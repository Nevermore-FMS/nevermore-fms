use log::info;
use std::{collections::HashMap, hash::Hash, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tonic::transport::Server;
use serde_derive::Serialize;

use crate::{field::Field, plugin::{api::NetworkConfiguratorApiImpl, rpc::network_configurator_api_server::NetworkConfiguratorApiServer}};

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
            let generic_api_impl = GenericApiImpl {
                plugin_manager: manager_clone,
                field: field.clone(),
            };
            let network_api_impl = NetworkConfiguratorApiImpl {
                field,
            };

            info!("Listening for gRPC plugins on {}", addr.clone());

            Server::builder()
                .add_service(GenericApiServer::new(generic_api_impl))
                .add_service(NetworkConfiguratorApiServer::new(network_api_impl))
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

    pub async fn get_plugins_metadata(&self) -> Vec<PluginMetadata> {
        let raw = self.raw.read().await;
        let mut out: Vec<PluginMetadata> = vec![];
        for (_, plugin) in raw.plugins.iter() {
            out.push(plugin.get_metadata().await.clone());
        }
        out
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

#[derive(Clone, Serialize)]
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
    pub async fn get_metadata(&self) -> PluginMetadata {
        let raw = self.raw.read().await;
        return raw.metadata.clone();
    }

    pub fn new(manager: PluginManager, metadata: PluginMetadata) -> Self {
        Plugin {
            raw: Arc::new(RwLock::new(RawPlugin { manager, metadata })),
        }
    }
}
