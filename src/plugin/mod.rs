use log::info;
use std::{collections::HashMap, hash::Hash, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tonic::transport::Server;

use crate::field::Field;

use self::{api::GenericApiImpl, rpc::generic_api_server::GenericApiServer};

pub mod rpc {
    tonic::include_proto!("plugin");
}

pub mod api;

pub struct RawPluginManager {
    plugins: HashMap<String, Plugin>,
}

#[derive(Clone)]
pub struct PluginManager {
    raw: Arc<RwLock<RawPluginManager>>,
}

impl PluginManager {
    pub fn new(field: Field) -> Self {
        let manager = PluginManager {
            raw: Arc::new(RwLock::new(RawPluginManager {
                plugins: HashMap::new(),
            })),
        };

        let manager_clone = manager.clone();
        tokio::spawn(async move {
            let addr: SocketAddr = "0.0.0.0:5276".parse().unwrap();
            let api_impl = GenericApiImpl {
                plugin_manager: manager_clone,
                field,
            };

            info!("Listening for gRPC plugins on {}", addr.clone());

            Server::builder()
                .add_service(GenericApiServer::new(api_impl))
                .serve(addr)
                .await
                .unwrap();
        });

        manager
    }

    pub async fn set_plugin(&self, meta: PluginMetadata) -> Plugin {
        let plugin = Plugin::new(self.clone(), meta.clone());

        let mut raw = self.raw.write().await;
        raw.plugins.insert(meta.id, plugin.clone());

        plugin
    }

    pub async fn get_plugin(&self, id: String) -> Option<Plugin> {
        let raw = self.raw.read().await;
        for (x, plugin) in raw.plugins.iter() {
            if x.clone() == id {
                return Some(plugin.clone());
            }
        }
        None
    }

    pub async fn remove_plugin(&self, id: String) -> Option<Plugin> {
        let mut raw = self.raw.write().await;
        let plugin = raw.plugins.remove(&id);
        plugin
    }
}

pub struct RawPlugin {
    manager: PluginManager,
    metadata: PluginMetadata,
}

#[derive(Clone)]
pub struct PluginMetadata {
    id: String,
    name: String,
    token: String,
}

#[derive(Clone)]
pub struct Plugin {
    raw: Arc<RwLock<RawPlugin>>,
}

impl Plugin {
    pub fn new(manager: PluginManager, metadata: PluginMetadata) -> Self {
        Plugin {
            raw: Arc::new(RwLock::new(RawPlugin { manager, metadata })),
        }
    }
}
