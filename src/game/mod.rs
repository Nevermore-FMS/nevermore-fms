pub mod deno_nevermore;

use crate::field::ThreadSafeField;
use crate::pub_sub::ThreadSafePubSub;
use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_core::{
    Extension, JsRuntime, RuntimeOptions
};
use deno_fetch::NoFetchPermissions;
use deno_timers::NoTimersPermission;
use deno_websocket::NoWebSocketPermissions;

pub struct DenoGameEngine {
    runtime: JsRuntime,
}

impl DenoGameEngine {
    pub fn new(field: ThreadSafeField, pub_sub: ThreadSafePubSub) -> anyhow::Result<Self> {
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
            crate::game::deno_nevermore::init(field, pub_sub), // This is the nevermore specific extension which adds functions.
        ];

        let runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            ..Default::default()
        });

        Ok(DenoGameEngine { runtime })
    }

    pub fn run_code(&mut self, id: String, code: String) -> anyhow::Result<()> {
        self.runtime.execute(format!("deno:{}.js", id).as_str(), code.as_str())?;

        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> anyhow::Result<()> {
        self.runtime.run_event_loop(false).await
    }
}
