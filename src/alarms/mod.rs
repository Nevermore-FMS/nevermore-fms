use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, bail};
use targets::is_target_in_scope;
use tokio::sync::RwLock;

pub mod targets;

/// FMSAlarmType indicates how the alarm will be displayed
/// The Fault type will also activate the associated System Stop for the target_scope (LStop or EStop)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FMSAlarmType {
    Info,
    Warning,
    Fault,
}

#[derive(Clone)]
pub struct FMSAlarm {
    pub id: String,
    pub alarm_type: FMSAlarmType,
    pub code: String,
    pub description: String,
    pub source_id: String,
    pub target_scope: String,
    pub timestamp: u64,
    pub released: bool,
    pub auto_clear: bool,
}

pub struct RawFMSAlarmHandler {
    active_alarms: Vec<FMSAlarm>,
    historic_alarms: Vec<FMSAlarm>,
}

#[derive(Clone)]
pub struct FMSAlarmHandler {
    raw: Arc<RwLock<RawFMSAlarmHandler>>,
}

impl FMSAlarmHandler {
    // Public API -->

    pub async fn active_alarms(&self) -> Vec<FMSAlarm> {
        let raw = self.raw.read().await;
        raw.active_alarms.clone()
    }

    pub async fn historic_alarms(&self) -> Vec<FMSAlarm> {
        let raw = self.raw.read().await;
        raw.historic_alarms.clone()
    }

    pub async fn throw_alarm(
        &self,
        alarm_type: FMSAlarmType,
        code: &str,
        description: &str,
        source_id: &str,
        target_scope: &str,
        require_release: bool,
        auto_clear: bool,
    ) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;

        for active_alarm in raw.active_alarms.clone() {
            if active_alarm.code == code {
                bail!("Alarm with code {} is already active", code);
            }
        }

        if !require_release && auto_clear {
            bail!("Cannot set flag auto_clear if release is not required");
        }

        let new_alarm = FMSAlarm {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            alarm_type,
            code: code.to_string(),
            description: description.to_string(),
            source_id: source_id.to_string(),
            target_scope: target_scope.to_string(),
            released: !require_release,
            auto_clear,
        };

        raw.active_alarms.push(new_alarm);

        Ok(())
    }

    pub async fn release_alarm(&self, code: &str) -> anyhow::Result<()> {
        let mut raw = self.raw.write().await;

        for active_alarm in raw.active_alarms.iter_mut() {
            if active_alarm.code == code {
                active_alarm.released = true;
                if active_alarm.auto_clear {
                    let code = active_alarm.code.clone();
                    drop(raw);
                    let _ = self.clear_alarm(code.as_str()).await;
                }
                return Ok(());
            }
        }

        bail!("No active alarm with code {} exists", code);
    }

    pub async fn clear_alarm(&self, code: &str) -> anyhow::Result<bool> {
        let mut raw = self.raw.write().await;

        let idx = raw
            .active_alarms
            .iter()
            .position(|alarm| alarm.code == code)
            .context("Invalid alarm code")?;
        if !raw.active_alarms.get(idx).unwrap().released {
            return Ok(false);
        }
        let alarm = raw.active_alarms.remove(idx);
        raw.historic_alarms.push(alarm);

        Ok(true)
    }

    /// Returns `true` if all active alarms could be cleared, and `false` if 
    /// any alarm could not be cleared
    pub async fn clear_all_alarms(&self) -> anyhow::Result<bool> {
        let alarms = self.active_alarms().await;
        let mut any_failed = false;
        for alarm in alarms {
            let alarm_cleared = self.clear_alarm(&alarm.code).await?;
            if !alarm_cleared {
                any_failed = true;
            }
        }
        
        Ok(!any_failed)
    }

    pub async fn is_target_faulted(&self, target: &str) -> bool {
        let raw = self.raw.write().await;
        for active_alarm in raw.active_alarms.clone() {
            if active_alarm.alarm_type == FMSAlarmType::Fault
                && is_target_in_scope(&active_alarm.target_scope, target)
            {
                return true;
            }
        }
        return false;
    }

    // Internal API -->

    pub(super) fn new() -> Self {
        let alarm_handler = RawFMSAlarmHandler {
            active_alarms: Vec::new(),
            historic_alarms: Vec::new(),
        };
        return Self {
            raw: Arc::new(RwLock::new(alarm_handler)),
        };
    }
}
