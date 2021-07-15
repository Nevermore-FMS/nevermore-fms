use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

type ThreadSafeNetworkConfiguratorMap = Arc<RwLock<HashMap<String, ThreadSafeNetworkConfigurator>>>;

type ThreadSafeNetworkConfigurator = Arc<RwLock<NetworkConfigurator>>;

struct NetworkConfigurator {
    
}