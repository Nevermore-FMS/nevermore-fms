pub mod targets;

use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, bail};

/// `FMSAlarmType` indicates how the alarm will be displayed.
/// `FMSAlarmType::Fault` will also activate the associated System Stop for the `target_scope` (LStop or EStop)
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

#[derive(Clone)]
pub struct FMSAlarmThrowable {
    pub alarm_type: FMSAlarmType,
    pub code: String,
    pub description: String,
    pub source_id: String,
    pub target_scope: String,
    pub require_release: bool,
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

    pub fn active_alarms(&self) -> Vec<FMSAlarm> {
        let raw = self.raw.read().unwrap();
        raw.active_alarms.clone()
    }

    pub fn historic_alarms(&self) -> Vec<FMSAlarm> {
        let raw = self.raw.read().unwrap();
        raw.historic_alarms.clone()
    }

    pub fn throw_alarm(&self, throwable: FMSAlarmThrowable) -> anyhow::Result<()> {
        let active_alarms = self.active_alarms();

        for active_alarm in active_alarms {
            if active_alarm.code == throwable.code {
                bail!("Alarm with code {} is already active", throwable.code);
            }
        }

        if !throwable.require_release && throwable.auto_clear {
            bail!("Cannot set flag auto_clear if release is not required");
        }

        let new_alarm = FMSAlarm {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            alarm_type: throwable.alarm_type,
            code: throwable.code,
            description: throwable.description,
            source_id: throwable.source_id,
            target_scope: throwable.target_scope,
            released: !throwable.require_release,
            auto_clear: throwable.auto_clear,
        };

        let mut raw = self.raw.write().unwrap();
        raw.active_alarms.push(new_alarm);

        Ok(())
    }

    pub fn release_alarm(&self, code: &str) -> anyhow::Result<()> {
        let mut raw = self.raw.write().unwrap();

        for active_alarm in &mut raw.active_alarms {
            if active_alarm.code == code {
                active_alarm.released = true;
                if active_alarm.auto_clear {
                    let code = active_alarm.code.clone();
                    drop(raw);
                    let _ = self.clear_alarm(code.as_str());
                }
                return Ok(());
            }
        }

        bail!("No active alarm with code {} exists", code);
    }

    pub fn clear_alarm(&self, code: &str) -> anyhow::Result<bool> {
        let mut raw = self.raw.write().unwrap();

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
    pub fn clear_all_alarms(&self) -> anyhow::Result<bool> {
        let alarms = self.active_alarms();
        let mut any_failed = false;
        for alarm in alarms {
            let alarm_cleared = self.clear_alarm(&alarm.code)?;
            if !alarm_cleared {
                any_failed = true;
            }
        }

        Ok(!any_failed)
    }

    pub fn is_target_faulted(&self, target: &str) -> bool {
        for active_alarm in self.active_alarms() {
            if active_alarm.alarm_type == FMSAlarmType::Fault
                && targets::is_target_in_scope(&active_alarm.target_scope, target)
            {
                return true;
            }
        }

        false
    }

    // Internal API -->

    pub(super) fn new() -> Self {
        let alarm_handler = RawFMSAlarmHandler {
            active_alarms: Vec::new(),
            historic_alarms: Vec::new(),
        };
        Self {
            raw: Arc::new(RwLock::new(alarm_handler)),
        }
    }
}
