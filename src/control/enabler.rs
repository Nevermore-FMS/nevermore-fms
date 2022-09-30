use async_trait::async_trait;

use crate::field::{driverstation::DriverStation, enums::AllianceStation};

pub type Enabler = Box<dyn EnablerTrait + Sync + Send>;

#[async_trait]
pub trait EnablerTrait {
    async fn is_ds_enabled(&self, ds: DriverStation) -> bool;
    fn name(&self) -> String;
}

pub struct AllEnabler {
    name: String,
    active: bool,
}
impl AllEnabler {
    pub fn new(name: String, active: bool) -> Enabler {
        Box::new(Self { name, active })
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
#[async_trait]
impl EnablerTrait for AllEnabler {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_enabled(&self, _ds: DriverStation) -> bool {
        self.active
    }
}

pub struct TeamNumberEnabler {
    name: String,
    approved_team_numbers: Vec<u16>,
}
impl TeamNumberEnabler {
    pub fn new(name: String, approved_team_numbers: Vec<u16>) -> Enabler {
        Box::new(Self {
            name,
            approved_team_numbers,
        })
    }
    pub fn add_team_number(&mut self, team_number: u16) {
        self.approved_team_numbers.push(team_number);
    }
    pub fn remove_team_number(&mut self, team_number: u16) {
        self.approved_team_numbers.retain(|&x| x != team_number);
    }
    pub fn clear_team_numbers(&mut self) {
        self.approved_team_numbers.clear();
    }
}
#[async_trait]
impl EnablerTrait for TeamNumberEnabler {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_enabled(&self, ds: DriverStation) -> bool {
        self.approved_team_numbers.contains(&ds.team_number().await)
    }
}

pub struct AllianceStationEnabler {
    name: String,
    approved_stations: Vec<AllianceStation>,
}
impl AllianceStationEnabler {
    pub fn new(name: String, approved_stations: Vec<AllianceStation>) -> Enabler {
        Box::new(Self {
            name,
            approved_stations,
        })
    }
    pub fn add_station(&mut self, station: AllianceStation) {
        self.approved_stations.push(station);
    }
    pub fn remove_station(&mut self, station: AllianceStation) {
        self.approved_stations.retain(|&x| x != station);
    }
    pub fn clear_stations(&mut self) {
        self.approved_stations.clear();
    }
}
#[async_trait]
impl EnablerTrait for AllianceStationEnabler {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_enabled(&self, ds: DriverStation) -> bool {
        self.approved_stations
            .contains(&ds.alliance_station().await)
    }
}
