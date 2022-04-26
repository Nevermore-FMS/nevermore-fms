use std::{sync::Arc, collections::HashMap};

use tokio::sync::RwLock;

use crate::field::driverstation::DriverStation;

use self::{enabler::SyncEnabler, estopper::SyncEstopper};

pub mod enabler;
pub mod estopper;


pub struct RawControlSystem {
    plugin_id_to_control_system: HashMap<String, PluginControlSystem>
}

pub struct PluginControlSystem {
    enablers: HashMap<String, SyncEnabler>,
    estoppers: HashMap<String, SyncEstopper>
}

#[derive(Clone)]
pub struct ControlSystem {
    raw: Arc<RwLock<RawControlSystem>>
}

impl ControlSystem {
    pub fn new() -> Self {
        ControlSystem { 
            raw: Arc::new(
                RwLock::new(
                    RawControlSystem { 
                        plugin_id_to_control_system: HashMap::new()
                    }
                )
            )
        }
    }

    pub async fn is_ds_enabled(&self, ds: DriverStation) -> bool {
        let mut enabled = false;
        for (_, control_system) in self.raw.read().await.plugin_id_to_control_system.iter() {
            for (_, enabler) in control_system.enablers.iter() {
                if !enabler.is_ds_enabled(ds.clone()).await {
                    return false;
                } else {
                    enabled = true;
                }
            }
        }

        return enabled;
    }

    pub async fn is_ds_estopped(&self, ds: DriverStation) -> bool {
        for (_, control_system) in self.raw.read().await.plugin_id_to_control_system.iter() {
            for (_, estopper) in control_system.estoppers.iter() {
                if estopper.is_ds_estopped(ds.clone()).await {
                    return true;
                }
            }
        }

        return true;
    }
}