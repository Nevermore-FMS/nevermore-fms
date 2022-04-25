use std::{sync::Arc, collections::HashMap, net::SocketAddr};
use log::info;
use tokio::sync::RwLock;
use tonic::transport::Server;

use crate::field::Field;

use self::{rpc::{PluginInfo, fms_server::FmsServer}, api::FmsImpl};

pub mod rpc {
    tonic::include_proto!("plugin");
}

pub mod api;

pub struct RawPluginManager {
    plugins: HashMap<String, PluginInfo>
}

#[derive(Clone)]
pub struct PluginManager {
    raw: Arc<RwLock<RawPluginManager>>
}

impl PluginManager {
    pub fn new(field: Field) -> Self {
        let manager = PluginManager {
            raw: Arc::new(
                RwLock::new(
                    RawPluginManager { 
                        plugins: HashMap::new()
                    }
                )
            )
        };

        let manager_clone = manager.clone();
        tokio::spawn(async move {
            let addr: SocketAddr = "0.0.0.0:5276".parse().unwrap();
            let api_impl = FmsImpl {
                plugin_manager: manager_clone,
                field
            };

            info!("Listening for gRPC plugins on {}", addr.clone());

            Server::builder()
                .add_service(FmsServer::new(api_impl))
                .serve(addr)
                .await.unwrap();
        });

        manager
    }


}