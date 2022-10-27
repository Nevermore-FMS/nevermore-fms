use log::info;
use std::{collections::HashMap, net::{SocketAddr, IpAddr}, sync::Arc};
use tokio::sync::{RwLock, broadcast};
use tonic::transport::Server;
use serde_derive::Serialize;
use rand::{Rng, distributions::Alphanumeric};

use crate::{field::Field, plugin::api::PluginApiImpl};

use self::rpc::{plugin_api_server::PluginApiServer, JsonRpcMessage, PluginRegistrationRequest, PluginRegistrationResponse};

pub mod rpc {
    tonic::include_proto!("plugin");
}

pub mod api;

pub struct RawPluginManager {
    plugins: HashMap<String, Plugin>,
    plugin_registration_token: String,
}

#[derive(Clone)]
pub struct PluginManager {
    raw: Arc<RwLock<RawPluginManager>>,

}

#[derive(Clone)]
pub struct PluginExtension {
    pub plugin: Plugin,
}

impl PluginManager {
    pub fn new(field: Field) -> Self {
        let manager = PluginManager {
            raw: Arc::new(RwLock::new(RawPluginManager {
                plugins: HashMap::new(),
                plugin_registration_token: rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(24)
                    .map(char::from)
                    .collect()
            })),
        };

        let manager_clone = manager.clone();
        tokio::spawn(async move {
            let addr: SocketAddr = "0.0.0.0:5276".parse().unwrap();
            let plugin_api_impl = PluginApiImpl {
                plugin_manager: manager_clone.clone(),
                field,
            };

            info!("Listening for gRPC plugins on {}", addr.clone());

            Server::builder()
                .add_service(PluginApiServer::new(plugin_api_impl))
                .serve(addr)
                .await
                .unwrap();
        });

        manager
    }

    pub async fn register_plugin(&self, req: PluginRegistrationRequest) -> anyhow::Result<PluginRegistrationResponse> {
        if req.plugin.is_none() {
            return Err(anyhow::anyhow!("No plugin in message!"));
        }
        let data = req.plugin.unwrap();
        let metadata = PluginMetadata{
            id: data.id.clone(),
            name: data.name,
            description: data.description,
            readme: data.readme,
            version: data.version,
            authors: data.authors,
            src_url: data.src_url,
            docs_url: data.docs_url,
        };

        let plugin = Plugin::new(metadata.clone());

        let mut raw = self.raw.write().await;
        raw.plugins.insert(data.id, plugin.clone());

        Ok(PluginRegistrationResponse { token: plugin.get_token().await })
    }

    pub async fn set_plugin(&self, meta: PluginMetadata) -> Plugin {
        let plugin = Plugin::new(meta.clone());

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

    pub async fn get_plugin_by_token(&self, token: String) -> Option<Plugin> {
        let raw = self.raw.read().await;
        for (_, plugin) in raw.plugins.iter() {
            if plugin.get_token().await == token {
                return Some(plugin.clone());
            }
        }
        None
    }

    pub async fn get_plugin_registration_token(&self) -> String {
        let raw = self.raw.read().await;
        raw.plugin_registration_token.clone()
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
    metadata: PluginMetadata,
    plugin_token: String,
    proxy: Option<PluginHTTPProxy>,
    message_channel: broadcast::Sender<JsonRpcMessage>
}

#[derive(Clone, Serialize)]
pub struct PluginHTTPProxy {
    protocol: String,
    ip_addr: IpAddr,
    port: u16
}

impl PluginHTTPProxy {
    pub fn generate_uri(&self, tail: String) -> String {
        format!("{}://{}:{}/{}", self.protocol, self.ip_addr.to_string(), self.port, tail)
    }
}

#[derive(Clone, Serialize)]
pub struct PluginMetadata {
    id: String,
    name: Option<String>,
    description: Option<String>,
    readme: Option<String>,
    version: Option<String>,
    authors: Vec<String>,
    src_url: Option<String>,
    docs_url: Option<String>
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

    pub async fn get_token(&self) -> String {
        let raw = self.raw.read().await;
        return raw.plugin_token.clone();
    }

    pub async fn get_http_proxy(&self) -> Option<PluginHTTPProxy> {
        let raw = self.raw.read().await;
        return raw.proxy.clone();
    }

    pub async fn publish(&self, msg: JsonRpcMessage) -> anyhow::Result<()> {
        let raw = self.raw.read().await;
        raw.message_channel.send(msg)?;
        Ok(())
    }

    pub async fn subscribe(&self) -> broadcast::Receiver<JsonRpcMessage> {
        let raw = self.raw.read().await;
        return raw.message_channel.subscribe();
    }

    pub fn new(metadata: PluginMetadata) -> Self {
        let (tx, _) = broadcast::channel(100);

        Plugin {
            raw: Arc::new(RwLock::new(RawPlugin { metadata, proxy: None, message_channel: tx, plugin_token: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(24)
                .map(char::from)
                .collect() })),
        }
    }
}
