use std::{collections::HashMap, fmt::Debug, sync::Arc};
use async_graphql::*;
use serde::{Serialize, Deserialize};

use tokio::sync::{RwLock, broadcast::{Receiver, Sender}};

pub type ThreadSafeNetworkConfiguratorMap = Arc<RwLock<NetworkConfiguratorMap>>;

pub type ThreadSafeNetworkConfigurator = Arc<RwLock<NetworkConfigurator>>;

pub struct NetworkConfiguratorMap {
    map: HashMap<String, ThreadSafeNetworkConfigurator>
}

impl NetworkConfiguratorMap {
    pub fn new() -> ThreadSafeNetworkConfiguratorMap {
        Arc::new(RwLock::new(Self{ map: HashMap::new() }))
    }

    pub fn register(&mut self, info: NetworkConfiguratorInfo) -> ThreadSafeNetworkConfigurator {
        let configurator = NetworkConfigurator::new(info.clone());
        self.map.insert(info.name.clone(), configurator.clone());
        configurator
    }

    pub fn get(&self, name: String) -> Option<&ThreadSafeNetworkConfigurator> {
        self.map.get(&name)
    }

    pub fn get_all(&self) -> Vec<&ThreadSafeNetworkConfigurator> {
        let mut out = Vec::new();
        for val in self.map.values() {
            out.push(val);
        }
        out
    }

    pub async fn get_info(&self, name: String) -> anyhow::Result<NetworkConfiguratorInfo> {
        Ok(self.map.get(&name).ok_or(anyhow::anyhow!("key doesn't exit"))?.read().await.info.clone())
    }

    pub async fn get_all_info(&self) -> Vec<NetworkConfiguratorInfo> {
        let mut out = Vec::new();
        for val in self.map.values() {
            out.push(val.read().await.info.clone());
        }
        out
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[derive(Debug, Clone)]
pub enum Reply {
    SUCCESS,
    ERROR(String)
}

#[derive(Debug, Clone, InputObject, Deserialize, Serialize)]
pub struct AllianceStationConfiguration {
    pub ssid: String,
    pub password: String
}

#[derive(Debug, Clone, InputObject, Deserialize, Serialize)]
pub struct AllianceStationToConfiguration {
    pub red1: AllianceStationConfiguration,
    pub red2: AllianceStationConfiguration,
    pub red3: AllianceStationConfiguration,
    pub blue1: AllianceStationConfiguration,
    pub blue2: AllianceStationConfiguration,
    pub blue3: AllianceStationConfiguration,
}

pub struct NetworkConfigurator {
    pub info: NetworkConfiguratorInfo,
    scan_pair: RequestReplyPair<(), Reply>,
    inital_configuration_pair: RequestReplyPair<(), Reply>,
    match_configuration_pair: RequestReplyPair<AllianceStationToConfiguration, Reply>
}

impl NetworkConfigurator {
    pub fn new(info: NetworkConfiguratorInfo) -> ThreadSafeNetworkConfigurator {
        let timeout = info.timeout;
        Arc::new(RwLock::new(Self{
            info,
            scan_pair: RequestReplyPair::new(timeout),
            inital_configuration_pair: RequestReplyPair::new(timeout),
            match_configuration_pair: RequestReplyPair::new(timeout)
        }))
    }

    pub async fn run_scan(&self) -> anyhow::Result<Reply> {
        Ok(self.scan_pair.request(()).await?)
    }

    pub fn subscribe_scan(&self) -> Receiver<()> {
        self.scan_pair.subscribe()
    }

    pub fn reply_scan(&self, reply: Reply) {
        self.scan_pair.reply(reply)
    }

    pub async fn run_initial_configuration(&self) -> anyhow::Result<Reply> {
        Ok(self.inital_configuration_pair.request(()).await?)
    }

    pub fn subscribe_initial_configuration(&self) -> Receiver<()> {
        self.inital_configuration_pair.subscribe()
    }

    pub fn reply_initial_configuration(&self, reply: Reply) {
        self.inital_configuration_pair.reply(reply)
    }

    pub async fn run_match_configuration(&self, alliance_station_to_configuration: AllianceStationToConfiguration) -> anyhow::Result<Reply> {
        Ok(self.match_configuration_pair.request(alliance_station_to_configuration).await?)
    }

    pub fn subscribe_match_configuration(&self) -> Receiver<AllianceStationToConfiguration> {
        self.match_configuration_pair.subscribe()
    }

    pub fn reply_match_configuration(&self, reply: Reply) {
        self.match_configuration_pair.reply(reply)
    }
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfiguratorInfo {
    pub name: String,
    pub readme: String,
    pub author: String,
    pub url: String,
    pub email: String,
    pub supported_hardware: Vec<String>,
    pub timeout: u64
}

pub struct RequestReplyPair<S, R> {
    request_sender: Sender<S>,
    reply_sender: Sender<R>,
    reply_timeout: u64 // In Seconds
}

impl<S, R> RequestReplyPair<S, R> where S: Clone, R: Clone {
    pub fn new(reply_timeout: u64) -> Self {
        let (request_sender, _) = tokio::sync::broadcast::channel::<S>(1);
        let (reply_sender, _) = tokio::sync::broadcast::channel::<R>(1);
        Self{
            request_sender,
            reply_sender,
            reply_timeout
        }
    }

    pub async fn request(&self, request: S) -> anyhow::Result<R> {
        self.request_sender.send(request).map_err(|err| anyhow::anyhow!(err.to_string()))?;
        let mut receiver = self.reply_sender.subscribe();
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(self.reply_timeout)) => {
                Err(anyhow::anyhow!("timed out"))
            }
            res = receiver.recv() => {
                Ok(res?)
            }
        }
    }

    pub fn subscribe(&self) -> Receiver<S> {
        self.request_sender.subscribe()
    }

    pub fn reply(&self, msg: R) {
        self.reply_sender.send(msg).ok();
    }
}