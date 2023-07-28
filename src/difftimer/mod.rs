use std::time::{Duration, SystemTime};

/// DiffTimer is a way to represent the game time remaining in a way that can easily be synced
/// between different displaying devices provided they all use NTP.
///
/// If started_at is None, then the timer is currently frozen and time_remaining represents the time that should be displayed
///
/// If started_at is Some, then the timer is currently running and time_remaining represents the time
/// that the clock had at the time specified by started_at.

#[derive(Clone)]
pub struct DiffTimer {
    started_at: Option<SystemTime>,
    time_remaining: Duration,
}

impl DiffTimer {
    pub fn new(time_remaining: Duration, start_running: bool) -> DiffTimer {
        let mut started_at: Option<SystemTime> = None;
        if start_running {
            started_at = Some(SystemTime::now())
        }
        DiffTimer {
            time_remaining: time_remaining,
            started_at,
        }
    }

    pub fn is_running(&self) -> bool {
        self.started_at != None
    }

    pub fn current_time_remaining(&self) -> Duration {
        if self.is_running() {
            let time_passed = SystemTime::now()
                .duration_since(self.started_at.unwrap())
                .unwrap_or(Duration::ZERO);
            if time_passed > self.time_remaining {
                return Duration::ZERO;
            } else {
                return self.time_remaining - time_passed;
            }
        } else {
            return self.time_remaining;
        }
    }

    pub fn start(&self) -> DiffTimer {
        DiffTimer { 
            started_at: Some(SystemTime::now()),
            time_remaining: self.time_remaining
        }
    }

    pub fn stop(&self) -> DiffTimer {
        DiffTimer { 
            started_at: None,
            time_remaining: self.current_time_remaining()
        }
    }
}