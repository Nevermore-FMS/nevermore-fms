pub mod deno_nevermore;
pub mod deno_pubsub;
pub mod deno_database;
pub mod deno_network;

use crate::field::ThreadSafeField;
use crate::pub_sub::ThreadSafePubSub;
use crate::plugin::deno_nevermore::LogMessage;
use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_core::{Extension, Snapshot};
use deno_core::{InspectorSessionProxy, JsRuntime, RuntimeOptions};
use deno_fetch::NoFetchPermissions;
use deno_net::NoNetPermissions;
use deno_timers::NoTimersPermission;
use deno_websocket::NoWebSocketPermissions;
use futures::channel::mpsc::UnboundedSender;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;

pub type ThreadSafeDenoPluginRuntime = Arc<RwLock<DenoPluginRuntime>>;

pub struct DenoPluginRuntime {
    runtime: JsRuntime,
    inspector_sender: UnboundedSender<InspectorSessionProxy>,
}

impl DenoPluginRuntime {
    pub fn new(
        field: ThreadSafeField,
        pub_sub: ThreadSafePubSub,
        log_channel: Sender<LogMessage>,
    ) -> ThreadSafeDenoPluginRuntime {
        let perm_ext = Extension::builder()
            .state(move |state| {
                state.put::<NoFetchPermissions>(NoFetchPermissions {});
                state.put::<NoWebSocketPermissions>(NoWebSocketPermissions {});
                state.put::<NoTimersPermission>(NoTimersPermission {});
                state.put::<NoNetPermissions>(NoNetPermissions {});
                Ok(())
            })
            .build();

        let extensions = vec![
            deno_webidl::init(),
            deno_console::init(),
            deno_url::init(),
            deno_web::init(Default::default(), Default::default()),
            deno_fetch::init::<NoFetchPermissions>("nevermore".to_owned(), None, None),
            deno_net::init::<NoNetPermissions>(false),
            deno_websocket::init::<NoWebSocketPermissions>("nevermore".to_owned(), None),
            deno_crypto::init(None),
            deno_timers::init::<NoTimersPermission>(),
            deno_broadcast_channel::init(InMemoryBroadcastChannel::default(), false),
            perm_ext,
            deno_nevermore::init(field, log_channel.clone()),
            deno_pubsub::init(pub_sub),
            deno_database::init(),
        ];

        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            startup_snapshot: Some(Snapshot::Static(include_bytes!(
                "v8_snapshots/SNAPSHOT.bin"
            ))),
            ..Default::default()
        });

        runtime
            .execute_script("deno:bootstrap.js", include_str!("bootstrap.js"))
            .ok();

        let inspector_sender = runtime.inspector().get_session_sender();

        Arc::new(RwLock::new(Self {
            runtime,
            inspector_sender,
        }))
    }

    pub fn run_code(&mut self, id: String, code: String) -> anyhow::Result<()> {
        self.runtime
            .execute_script(format!("deno:{}.js", id).as_str(), code.as_str())?;

        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> anyhow::Result<()> {
        self.runtime.run_event_loop(false).await
    }

    pub fn get_session_sender(&self) -> UnboundedSender<InspectorSessionProxy> {
        self.inspector_sender.clone()
    }
}