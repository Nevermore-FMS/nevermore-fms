use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::field::driverstation::DriverStation;

use self::{enabler::Enabler, estopper::Estopper, fault::Fault};

pub mod enabler;
pub mod estopper;
pub mod fault;

pub struct RawControlSystem {
    plugin_id_to_control_system: HashMap<String, PluginControlSystem>,
}

#[derive(Clone)]
pub struct ControlSystem {
    raw: Arc<RwLock<RawControlSystem>>,
}

pub struct PluginControlSystem {
    enablers: HashMap<String, Enabler>,
    estoppers: HashMap<String, Estopper>,
    faults: HashMap<String, Fault>,
}



impl ControlSystem {
    pub fn new() -> Self {
        ControlSystem {
            raw: Arc::new(RwLock::new(RawControlSystem {
                plugin_id_to_control_system: HashMap::new(),
            })),
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
        if self.active_faults().await.len() > 0 {
            return true
        }

        for (_, control_system) in self.raw.read().await.plugin_id_to_control_system.iter() {
            for (_, estopper) in control_system.estoppers.iter() {
                if estopper.is_ds_estopped(ds.clone()).await {
                    return true;
                }
            }
        }

        return false;
    }

    pub async fn active_faults(&self) -> Vec<Fault> {
        let mut faults: Vec<Fault> = Vec::new();
        for (_, control_system) in self.raw.read().await.plugin_id_to_control_system.iter() {
            for (_, fault) in control_system.faults.iter() {
                if fault.active() {
                    faults.push(fault.clone())
                }
            }
        }
        faults
    }

    pub async fn register_plugin(&self, plugin_id: String) {
        let mut raw = self.raw.write().await;
        raw.plugin_id_to_control_system.insert(plugin_id, PluginControlSystem{
            enablers: HashMap::new(),
            estoppers: HashMap::new(),
            faults: HashMap::new()
        });
    }

    pub async fn deregister_plugin(&self, plugin_id: String) {
        let mut raw = self.raw.write().await;
        raw.plugin_id_to_control_system.remove(&plugin_id);
    }

    pub async fn update_enabler(&self, plugin_id: String, enabler_id: String, enabler: Enabler) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.enablers.insert(enabler_id, enabler);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn remove_enabler(&self, plugin_id: String, enabler_id: String) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.enablers.remove(&enabler_id);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn update_estopper(&self, plugin_id: String, estopper_id: String, estopper: Estopper) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.estoppers.insert(estopper_id, estopper);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn remove_estopper(&self, plugin_id: String, estopper_id: String) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.estoppers.remove(&estopper_id);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn register_fault(&self, plugin_id: String, fault_id: String, fault: Fault) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.faults.insert(fault_id, fault);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn deregister_fault(&self, plugin_id: String, fault_id: String) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            plugin_cs.faults.remove(&fault_id);
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn activate_fault(&self, plugin_id: String, fault_id: String) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            if let Some(fault) = plugin_cs.faults.get_mut(&fault_id) {
                fault.activate()
            } else {
                return Err(anyhow::anyhow!("Fault is not registered"));
            }
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }

    pub async fn clear_fault(&self, plugin_id: String, fault_id: String) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;
        if let Some(plugin_cs) = raw.plugin_id_to_control_system.get_mut(&plugin_id) {
            if let Some(fault) = plugin_cs.faults.get_mut(&fault_id) {
                fault.clear()
            } else {
                return Err(anyhow::anyhow!("Fault is not registered"));
            }
        } else {
            return Err(anyhow::anyhow!("Plugin is not registered"));
        }
        Ok(())
    }
    

}
