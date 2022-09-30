use async_trait::async_trait;

use crate::field::{driverstation::DriverStation, enums::AllianceStation};

pub type Estopper = Box<dyn EstopperTrait + Sync + Send>;

#[async_trait]
pub trait EstopperTrait {
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool;
    fn name(&self) -> String;
}

pub struct AllEstopper {
    name: String,
    active: bool,
}
impl AllEstopper {
    pub fn new(name: String, active: bool) -> Estopper {
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
impl EstopperTrait for AllEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, _ds: DriverStation) -> bool {
        self.active
    }
}

pub struct TeamNumberEstopper {
    name: String,
    estopped_team_numbers: Vec<u16>,
}
impl TeamNumberEstopper {
    pub fn new(name: String, estopped_team_numbers: Vec<u16>) -> Estopper {
        Box::new(Self {
            name,
            estopped_team_numbers,
        })
    }
    pub fn add_team_number(&mut self, team_number: u16) {
        self.estopped_team_numbers.push(team_number);
    }
    pub fn remove_team_number(&mut self, team_number: u16) {
        self.estopped_team_numbers.retain(|&x| x != team_number);
    }
    pub fn clear_team_numbers(&mut self) {
        self.estopped_team_numbers.clear();
    }
}
#[async_trait]
impl EstopperTrait for TeamNumberEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool {
        self.estopped_team_numbers.contains(&ds.team_number().await)
    }
}

pub struct AllianceStationEstopper {
    name: String,
    estopped_stations: Vec<AllianceStation>,
}

impl AllianceStationEstopper {
    pub fn new(name: String, estopped_stations: Vec<AllianceStation>) -> Estopper {
        Box::new(Self {
            name,
            estopped_stations,
        })
    }
    pub fn add_station(&mut self, station: AllianceStation) {
        self.estopped_stations.push(station);
    }
    pub fn remove_station(&mut self, station: AllianceStation) {
        self.estopped_stations.retain(|&x| x != station);
    }
    pub fn clear_stations(&mut self) {
        self.estopped_stations.clear();
    }
}
#[async_trait]
impl EstopperTrait for AllianceStationEstopper {
    fn name(&self) -> String {
        self.name.clone()
    }
    async fn is_ds_estopped(&self, ds: DriverStation) -> bool {
        self.estopped_stations
            .contains(&ds.alliance_station().await)
    }
}
