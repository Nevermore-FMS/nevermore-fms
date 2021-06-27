pub mod deno_nevermore;

use deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_core::error::AnyError;
use deno_core::futures::StreamExt;
use deno_core::{
    Extension, FsModuleLoader, JsRuntime, ModuleLoader, ModuleSourceFuture, ModuleSpecifier,
    NoopModuleLoader, OpState, RuntimeOptions, Snapshot,
};
use deno_fetch::NoFetchPermissions;
use deno_timers::NoTimersPermission;
use deno_websocket::NoWebSocketPermissions;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

#[derive(Default)]
struct ModsLoader;

impl ModuleLoader for ModsLoader {
    fn resolve(
        &self,
        _op_state: Rc<RefCell<OpState>>,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, AnyError> {
        assert_eq!(specifier, "file:///main.js");
        assert_eq!(referrer, ".");
        let s = deno_core::resolve_import(specifier, referrer).unwrap();
        Ok(s)
    }

    fn load(
        &self,
        _op_state: Rc<RefCell<OpState>>,
        _module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        unreachable!()
    }
}

pub struct DenoGameEngine {
    runtime: JsRuntime,
}

impl DenoGameEngine {
    pub fn new() -> anyhow::Result<Self> {
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
            crate::game::deno_nevermore::init(), // This is the nevermore specific extension which adds functions.
        ];

        let loader = std::rc::Rc::new(ModsLoader::default());
        let runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(loader),
            extensions,
            ..Default::default()
        });

        Ok(DenoGameEngine { runtime })
    }

    pub async fn start(&mut self, code: String) -> anyhow::Result<()> {
        let module = self
            .runtime
            .load_module(
                &deno_core::resolve_url("file:///main.js")?,
                Option::Some(code),
            )
            .await?;
        let mut receiver = self.runtime.mod_evaluate(module);

        tokio::select! {
          event_loop_result = self.runtime.run_event_loop(false) => {
            event_loop_result?;
            let maybe_result = receiver.next().await;
            maybe_result.expect("Module evaluation result not provided.")
          }
        }
    }

    fn get_helper_code(&self) {}
}
