use std::{
    collections::HashMap,
    io::Cursor,
    net::IpAddr,
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::{anyhow, bail};
use chrono::Utc;
use cidr::AnyIpCidr;
use log::*;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_util::sync::CancellationToken;

use crate::alarms::FMSAlarmType;

use super::{
    Field,
    connection::DriverStationConnection,
    enums::{AllianceStation, Mode, VersionData, VersionType},
};

struct RawDriverStation {
    parent: DriverStations,
    team_number: u16,
    alliance_station: AllianceStation,
    commanded_enabled: bool,
    expected_ip: Option<AnyIpCidr>,
    active_connection: Option<DriverStationConnection>,
    confirmed_state: Option<DriverStationConfirmedState>,
    log_data: Vec<DriverStationLogData>,
    versions: HashMap<VersionType, VersionData>,
    log_messages: Vec<DriverStationLogMessage>,
}

#[derive(Clone)]
pub struct DriverStation {
    raw: Arc<RwLock<RawDriverStation>>,
}

impl DriverStation {
    // Public API -->

    pub fn parent(&self) -> DriverStations {
        let raw = self.raw.read().unwrap();
        raw.parent.clone()
    }

    pub fn alarm_target(&self) -> String {
        format!("fms.field.driverstations.{}", self.alliance_station())
    }

    pub fn team_number(&self) -> u16 {
        let raw = self.raw.read().unwrap();
        raw.team_number
    }

    pub fn alliance_station(&self) -> AllianceStation {
        let raw = self.raw.read().unwrap();
        raw.alliance_station
    }

    pub fn commanded_enabled(&self) -> bool {
        let raw = self.raw.read().unwrap();
        raw.commanded_enabled
    }

    pub fn enabled(&self) -> bool {
        let faulted = self
            .parent()
            .get_field()
            .alarm_handler()
            .is_target_faulted(self.alarm_target().as_str());
        let commanded_enabled = self.commanded_enabled();
        commanded_enabled && !faulted
    }

    pub fn expected_ip(&self) -> Option<AnyIpCidr> {
        let raw = self.raw.read().unwrap();
        raw.expected_ip
    }

    pub fn active_connection(&self) -> Option<DriverStationConnection> {
        let raw = self.raw.read().unwrap();
        raw.active_connection.clone()
    }

    pub fn confirmed_state(&self) -> Option<DriverStationConfirmedState> {
        let raw = self.raw.read().unwrap();
        raw.confirmed_state
    }

    pub fn log_data(&self) -> Vec<DriverStationLogData> {
        let raw = self.raw.read().unwrap();
        raw.log_data.clone()
        //TODO Don't store in RAM, read from DB instead
    }

    pub fn log_messages(&self) -> Vec<DriverStationLogMessage> {
        let raw = self.raw.read().unwrap();
        raw.log_messages.clone()
        //TODO Don't store in RAM, read from DB instead
    }

    pub fn versions(&self) -> HashMap<VersionType, VersionData> {
        let raw = self.raw.read().unwrap();
        raw.versions.clone()
    }

    pub fn update_expected_ip(&self, expected_ip: AnyIpCidr) {
        let mut raw = self.raw.write().unwrap();
        raw.expected_ip = Option::Some(expected_ip);
        info!(
            "Expected ip of {} set to {}",
            raw.team_number,
            raw.expected_ip.unwrap()
        );
    }

    // Internal API -->

    fn new(parent: DriverStations, team_number: u16, alliance_station: AllianceStation) -> Self {
        let driverstation = RawDriverStation {
            parent,
            team_number,
            alliance_station,
            commanded_enabled: false,
            expected_ip: None,
            active_connection: None,
            confirmed_state: None,
            log_data: Vec::new(),
            versions: HashMap::new(),
            log_messages: Vec::new(),
        };

        Self {
            raw: Arc::new(RwLock::new(driverstation)),
        }
    }

    pub(super) fn set_version(&self, version_type: VersionType, version: VersionData) {
        let mut raw = self.raw.write().unwrap();
        raw.versions.insert(version_type, version);
    }

    pub(super) fn record_log_data(&self, log_data: DriverStationLogData) {
        let mut raw = self.raw.write().unwrap();
        raw.log_data.push(log_data);
        //TODO Don't store in RAM, write to DB instead
    }

    pub(super) fn add_log_message(&self, log_message: DriverStationLogMessage) {
        let mut raw = self.raw.write().unwrap();
        raw.log_messages.push(log_message);
        //TODO Don't store in RAM, write to DB instead
    }

    pub(super) fn set_confirmed_state(&self, confirmed_state: Option<DriverStationConfirmedState>) {
        let mut raw = self.raw.write().unwrap();
        raw.confirmed_state = confirmed_state;
    }

    pub(super) fn remove_active_connection(&self) -> Option<DriverStationConnection> {
        let mut raw = self.raw.write().unwrap();
        raw.active_connection.take()
    }

    pub(super) fn set_active_connection(&self, active_connection: DriverStationConnection) {
        let mut raw = self.raw.write().unwrap();
        raw.active_connection = Some(active_connection);
    }

    pub(super) fn set_commanded_enabled(&self, enabled: bool) {
        let mut raw = self.raw.write().unwrap();
        raw.commanded_enabled = enabled;
    }

    async fn tick(&self) {
        // Respond to active faults
        if self
            .parent()
            .get_field()
            .alarm_handler()
            .is_target_faulted(self.alarm_target().as_str())
        {
            self.set_commanded_enabled(false);
        }

        if let Some(conn) = self.active_connection()
            && conn.is_alive()
        {
            if Utc::now().signed_duration_since(conn.last_udp_packet_reception())
                > chrono::Duration::seconds(2)
            {
                conn.kill().await;
            } else {
                let udp_result = conn.send_udp_message().await;
                if let Err(e) = udp_result {
                    error!(
                        "Error sending udp message to driver station{}: {}",
                        self.team_number(),
                        e
                    );
                };
            }
        }
    }
}

//Represents all driverstations (connected and not connected), their sockets, and manages various ways to index them
pub struct RawDriverStations {
    field: Option<Field>,
    all_driverstations: Vec<DriverStation>,
}

#[derive(Clone)]
pub struct DriverStations {
    raw: Arc<RwLock<RawDriverStations>>,
}

impl DriverStations {
    // Public API -->

    pub fn add_driverstation(
        &self,
        team_number: u16,
        alliance_station: AllianceStation,
    ) -> anyhow::Result<DriverStation> {
        if self.get_driverstation_by_team_number(team_number).is_some() {
            bail!(
                "Driverstation with team number {} already exists",
                team_number
            );
        }

        if self
            .get_driverstation_by_position(alliance_station)
            .is_some()
        {
            bail!(
                "Driverstation already exists in alliance station {:?}",
                alliance_station
            );
        }

        let driverstation = DriverStation::new(self.clone(), team_number, alliance_station);

        let mut raw_driverstations = self.raw.write().unwrap();
        raw_driverstations
            .all_driverstations
            .push(driverstation.clone());

        info!(
            "Added driverstation {} to {}",
            driverstation.team_number(),
            driverstation.alliance_station()
        );

        Ok(driverstation)
    }

    pub async fn delete_driverstation(&self, team_number: u16) -> anyhow::Result<()> {
        let all_driverstations = self.get_all_driverstations();
        let mut new_driverstations: Vec<DriverStation> = Vec::new();

        for ds in all_driverstations.iter() {
            if ds.team_number() != team_number {
                new_driverstations.push(ds.clone());
            } else if let Some(conn) = ds.active_connection() {
                info!("Deleted driverstation {}", team_number);
                conn.kill().await;
            }
        }

        if all_driverstations.len() > new_driverstations.len() {
            let mut raw_driverstations = self.raw.write().unwrap();
            raw_driverstations.all_driverstations = new_driverstations;
            Ok(())
        } else {
            Err(anyhow!(
                "Failed to delete driverstation {} - driverstation does not exist",
                team_number
            ))
        }
    }

    pub fn get_driverstation_by_team_number(&self, team_number: u16) -> Option<DriverStation> {
        let all_driverstations = self.get_all_driverstations();
        for ds in all_driverstations.iter() {
            if ds.team_number() == team_number {
                return Some(ds.clone());
            }
        }

        None
    }

    pub fn get_driverstation_by_position(
        &self,
        alliance_station: AllianceStation,
    ) -> Option<DriverStation> {
        let all_driverstations = self.get_all_driverstations();
        for ds in all_driverstations.iter() {
            if ds.alliance_station() == alliance_station {
                return Some(ds.clone());
            }
        }

        None
    }

    pub fn get_all_driverstations(&self) -> Vec<DriverStation> {
        let raw_driverstations = self.raw.read().unwrap();
        raw_driverstations.all_driverstations.clone()
    }

    pub fn get_field(&self) -> Field {
        let raw_driverstations = self.raw.read().unwrap();
        if let Some(field) = raw_driverstations.field.clone() {
            field
        } else {
            panic!("Driverstations::get_field() used too early");
        }
    }

    // Internal API -->

    pub(super) fn new(field: Option<Field>) -> Self {
        let driverstations = RawDriverStations {
            field,
            all_driverstations: Vec::new(),
        };

        Self {
            raw: Arc::new(RwLock::new(driverstations)),
        }
    }

    pub(super) async fn run(self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let task_handle = tokio::task::Builder::new()
            .name("DriverStations tick loop")
            .spawn(self.tick_loop(cancellation_token))?;

        task_handle.await??;

        Ok(())
    }

    pub(super) fn set_field(&self, field: Field) -> anyhow::Result<()> {
        let mut raw_driverstations = self.raw.write().unwrap();
        if raw_driverstations.field.is_some() {
            bail!("Field already set");
        }
        raw_driverstations.field = Some(field);
        Ok(())
    }

    async fn tick_loop(self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let interval_tick_loop = async {
            loop {
                interval.tick().await;
                self.tick().await;
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => Ok(()),
            _ = interval_tick_loop => Err(anyhow::anyhow!("Tick loop closed unexpectedly")),
        }
    }

    async fn tick(&self) {
        let all_driverstations = self.get_all_driverstations();
        let field = self.get_field();
        for ds in all_driverstations {
            // Throw conditional faults
            if ds.enabled() && field.is_safe() {
                let _ = field.alarm_handler().throw_alarm(
                    FMSAlarmType::Fault,
                    "FIELD_SAFE_MISMATCH",
                    "Driver Station is set to ENABLED but field SAFE flag was set. Invalid state.",
                    "fms.field.driverstations",
                    "fms.field",
                    true,
                    false,
                );
            }
            ds.tick().await;
        }
    }

    pub(super) async fn decode_udp_message(&self, buffer: Vec<u8>) -> anyhow::Result<()> {
        let mut reader = Cursor::new(buffer);

        let _sequence_num = reader.read_u16().await?; //TODO Track Packet loss
        let _comm_version = reader.read_u8().await?;
        let status_byte = reader.read_u8().await?;
        let team_number = reader.read_u16().await?;
        let battery_byte = reader.read_u16().await?;
        //TODO Handle Tags

        //Status byte info
        let is_emergency_stopped = (status_byte >> 7 & 0x01) == 1;
        let robot_communications_active = (status_byte >> 5 & 0x01) == 1;
        let can_ping_radio = (status_byte >> 4 & 0x01) == 1;
        let can_ping_rio = (status_byte >> 3 & 0x01) == 1;
        let is_enabled = (status_byte >> 2 & 0x01) == 1;
        let mode = Mode::from_byte(status_byte & 0x03);

        let battery_voltage =
            (battery_byte >> 8 & 0xff) as f32 + ((battery_byte & 0xff) as f32 / 256.0);

        let confirmed_state = DriverStationConfirmedState {
            is_emergency_stopped,
            robot_communications_active,
            can_ping_radio,
            can_ping_rio,
            is_enabled,
            mode,
            team_number,
            battery_voltage,
        };

        if let Some(ds) = self.get_driverstation_by_team_number(team_number) {
            ds.set_confirmed_state(Some(confirmed_state));
            if let Some(active_connection) = ds.active_connection() {
                active_connection.update_last_udp_packet_reception(Utc::now())
            }
        } else {
            warn!(
                "Received a packet from a driver station that is not in the list of known driver stations. Team Number: {}",
                team_number
            );
        }

        Ok(())
    }

    pub(super) async fn handle_tcp_stream(
        &self,
        tcp_stream: TcpStream,
        ip_address: IpAddr,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let ds_connection = DriverStationConnection::new(ip_address, self.get_field());
        ds_connection.run(tcp_stream, cancellation_token).await
    }
}

#[derive(Clone, Copy, Default)]
pub struct DriverStationConfirmedState {
    pub is_emergency_stopped: bool,
    pub robot_communications_active: bool,
    pub can_ping_radio: bool,
    pub can_ping_rio: bool,
    pub is_enabled: bool,
    pub mode: Mode,
    pub team_number: u16,
    pub battery_voltage: f32,
}

#[derive(Clone, Debug)]
pub struct DriverStationLogMessage {
    pub timestamp: u64,
    pub local_timestamp: u64,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct DriverStationLogData {
    pub timestamp: u64,
    pub trip_time: u8,
    pub lost_packets: u8,
    pub voltage: f32,
    pub brownout: bool,
    pub watchdog: bool,
    pub ds_teleop: bool,
    pub ds_auto: bool,
    pub ds_disable: bool,
    pub robot_teleop: bool,
    pub robot_auto: bool,
    pub robot_disable: bool,
    pub can_utilization: u8,
    pub signal: u8,
    pub bandwidth: f32,
}
