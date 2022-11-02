use std::{net::{IpAddr, SocketAddr}, sync::Arc};

use tokio::sync::RwLock;

use crate::{field::Field, plugin::PluginManager, web::start_web};

struct RawApplication {
    pub field: Field,
    #[allow(dead_code)] // This is not dead code, and prevents the plugin manager from being dropped.
    pub plugin_manager: PluginManager,
    running_signal: async_channel::Receiver<()>,
}

#[derive(Clone)]
pub struct Application {
    raw: Arc<RwLock<RawApplication>>,
}

impl Application {
    // Public API -->
    pub async fn terminate(&self) {
        let raw_app = self.raw.read().await;
        raw_app.field.terminate().await;
    }

    pub async fn wait_for_terminate(&self) {
        let raw_app = self.raw.read().await;
        let running_signal = raw_app.running_signal.clone();
        drop(raw_app);
        let _ = running_signal.recv().await;
    }

    pub async fn field(&self) -> Field {
        let raw_app = self.raw.read().await;
        raw_app.field.clone()
    }

    // Internal API -->
    pub(super) async fn new(ds_address: IpAddr, web_address: SocketAddr) -> anyhow::Result<Self> {
        let field = Field::new(String::from("DFLT"), ds_address).await?;

        let plugin_manager = PluginManager::new(field.clone());

        start_web(field.clone(), plugin_manager.clone(), web_address).await?;
        
        let (indicate_running, running_signal) = async_channel::bounded(1);

        let application = RawApplication {
            field,
            running_signal,
            plugin_manager
        };

        let wait_field = application.field.clone();
        tokio::spawn(async move {
            wait_field.wait_for_terminate().await;
            drop(indicate_running);
        });

        let application = Self {
            raw: Arc::new(RwLock::new(application)),
        };

        Ok(application)
    }
}
