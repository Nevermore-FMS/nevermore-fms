pub mod deno_nevermore;

use std::sync::Arc;
use crate::field::ThreadSafeField;
use crate::game::deno_nevermore::LogMessage;
use crate::pub_sub::ThreadSafePubSub;
use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_core::{Extension, Snapshot};
use deno_core::{InspectorSessionProxy, JsRuntime, RuntimeOptions};
use deno_fetch::NoFetchPermissions;
use deno_timers::NoTimersPermission;
use deno_websocket::NoWebSocketPermissions;
use futures::channel::mpsc::UnboundedSender;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;

pub type ThreadSafeDenoWorker = Arc<Mutex<DenoWorker>>;

pub struct DenoWorker {
    runtime: JsRuntime,
    inspector_sender: Option<UnboundedSender<InspectorSessionProxy>>,
}

impl DenoWorker {
    pub fn new(
        field: ThreadSafeField,
        pub_sub: ThreadSafePubSub,
        attach_inspector: bool,
        log_channel: Sender<LogMessage>,
    ) -> ThreadSafeDenoWorker {
        let perm_ext = Extension::builder()
            .state(move |state| {
                state.put::<NoFetchPermissions>(NoFetchPermissions {});
                state.put::<NoWebSocketPermissions>(NoWebSocketPermissions {});
                state.put::<NoTimersPermission>(NoTimersPermission {});
                Ok(())
            })
            .build();

        let extensions = vec![
            deno_webidl::init(),
            deno_console::init(),
            deno_url::init(),
            deno_web::init(Default::default(), Default::default()),
            deno_fetch::init::<NoFetchPermissions>("nevermore".to_owned(), None),
            deno_websocket::init::<NoWebSocketPermissions>("nevermore".to_owned(), None),
            deno_crypto::init(None),
            deno_timers::init::<NoTimersPermission>(),
            deno_broadcast_channel::init(InMemoryBroadcastChannel::default(), false),
            perm_ext,
            crate::game::deno_nevermore::init(field, pub_sub, log_channel.clone()), // This is the nevermore specific extension which adds functions.
        ];

        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            attach_inspector,
            startup_snapshot: Some(Snapshot::Static(include_bytes!("v8_snapshots/SNAPSHOT.bin"))),
            ..Default::default()
        });

        runtime.execute("deno:bootstrap.js", include_str!("bootstrap.js")).ok();

        let inspector_sender = if attach_inspector {
            let maybe_inspector = runtime.inspector();
            Some(maybe_inspector.unwrap().get_session_sender())
        } else {
            None
        };

        Arc::new(Mutex::new(Self {
            runtime,
            inspector_sender,
        }))
    }

    pub fn run_code(&mut self, id: String, code: String) -> anyhow::Result<()> {
        self.runtime
            .execute(format!("deno:{}.js", id).as_str(), code.as_str())?;

        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> anyhow::Result<()> {
        self.runtime.run_event_loop(false).await
    }

    pub fn get_session_sender(&self) -> Option<UnboundedSender<InspectorSessionProxy>> {
        self.inspector_sender.clone()
    }
}
