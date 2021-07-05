pub mod deno_nevermore;
pub mod inspector_server;


use std::net::SocketAddr;
use std::rc::Rc;
use std::sync::Arc;

use crate::field::ThreadSafeField;
use crate::game::deno_nevermore::LogMessage;
use crate::pub_sub::ThreadSafePubSub;
use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_core::{Extension, JsRuntime, RuntimeOptions};
use deno_fetch::NoFetchPermissions;
use deno_timers::NoTimersPermission;
use deno_websocket::NoWebSocketPermissions;
use inspector_server::InspectorServer;
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use log::info;

pub type ThreadSafeDenoWorker = Arc<Mutex<DenoWorker>>;

pub struct DenoWorker {
    runtime: JsRuntime,
    inspector_server: Option<InspectorServer>,
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
            js_error_create_fn: Some(Rc::new(move |core_js_error| {
                core_js_error.into()
              })),
            extensions,
            attach_inspector,
            ..Default::default()
        });

        let inspector_server = if attach_inspector {
            let listener: SocketAddr = "127.0.0.1:9229".parse().unwrap();
            let inspector_maybe = runtime.inspector();
            let inspector = inspector_maybe.unwrap();
            let inspector_server = InspectorServer::new(listener, "main".to_string());
            inspector_server.register_inspector(
                inspector.get_session_sender(),
                inspector.add_deregister_handler(),
            );
            Some(inspector_server)
        } else {
            None
        };

        Arc::new(Mutex::new(Self {
            runtime,
            inspector_server,
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
}
