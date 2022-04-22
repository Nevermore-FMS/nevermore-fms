use async_trait::async_trait;

use crate::field::{driverstation::DriverStation, enums::AllianceStation};

#[async_trait]
pub trait Estopper {
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool;
    fn name(&self) -> String;
}

struct AllEstopper {
    name: String,
    active: bool,
}
impl AllEstopper {
    pub fn new(name: String) -> Self {
        Self {
            name,
            active: true,
        }
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
#[async_trait]
impl Estopper for AllEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, _ds: DriverStation) -> bool {
        self.active
    }
}

struct TeamNumberEstopper {
    name: String,
    approved_team_numbers: Vec<u16>,
}
impl TeamNumberEstopper {
    pub fn new(name: String) -> Self {
        Self {
            name,
            approved_team_numbers: Vec::new(),
        }
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
impl Estopper for TeamNumberEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool {
        self.approved_team_numbers.contains(&ds.team_number().await)
    }
}

struct AllianceStationEstopper {
    name: String,
    approved_stations: Vec<AllianceStation>,
}
impl AllianceStationEstopper {
    pub fn new(name: String) -> Self {
        Self {
            name,
            approved_stations: Vec::new(),
        }
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
impl Estopper for AllianceStationEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool {
        self.approved_stations.contains(&ds.alliance_station().await)
    }
}