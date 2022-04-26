use std::{net::IpAddr, sync::Arc};

use tokio::sync::RwLock;

use crate::{field::Field, plugin::PluginManager};

struct RawApplication {
    pub field: Field,
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
    pub(super) async fn new(db_uri: Option<String>, ds_address: IpAddr) -> anyhow::Result<Self> {
        let field = Field::new(String::from("DFLT"), ds_address).await?;

        let plugin_manager = PluginManager::new(field.clone());

        let (indicate_running, running_signal) = async_channel::bounded(1);

        let application = RawApplication {
            field,
            running_signal,
            plugin_manager,
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
