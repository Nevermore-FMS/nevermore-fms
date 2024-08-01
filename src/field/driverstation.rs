use std::{collections::HashMap, io::Cursor, net::IpAddr, sync::Arc, time::Duration};

use anyhow::{bail, Context, Ok};
use chrono::Utc;
use cidr::AnyIpCidr;
use log::*;
use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    sync::{broadcast, RwLock},
    time,
};

use super::{
    connection::DriverStationConnection,
    enums::{AllianceStation, Mode, Version, VersionType},
    Field,
};

struct RawDriverStation {
    parent: DriverStations,
    team_number: u16,
    alliance_station: AllianceStation,
    mode: Mode,
    expected_ip: Option<AnyIpCidr>,
    active_connection: Option<DriverStationConnection>,
    confirmed_state: Option<ConfirmedState>,
    log_data: Option<LogData>,
    versions: HashMap<VersionType, Version>,
    log_messages: Vec<LogMessage>,
}

#[derive(Clone)]
pub struct DriverStation {
    raw: Arc<RwLock<RawDriverStation>>,
}

impl DriverStation {
    // Public API -->

    pub async fn parent(&self) -> DriverStations {
        let raw = self.raw.read().await;
        raw.parent.clone()
    }

    pub async fn team_number(&self) -> u16 {
        let raw = self.raw.read().await;
        raw.team_number
    }

    pub async fn alliance_station(&self) -> AllianceStation {
        let raw = self.raw.read().await;
        raw.alliance_station
    }

    pub async fn mode(&self) -> Mode {
        let raw = self.raw.read().await;
        raw.mode
    }

    pub async fn expected_ip(&self) -> Option<AnyIpCidr> {
        let raw = self.raw.read().await;
        raw.expected_ip
    }

    pub async fn active_connection(&self) -> Option<DriverStationConnection> {
        let raw = self.raw.read().await;
        raw.active_connection.clone()
    }

    pub async fn log_messages(&self) -> Vec<LogMessage> {
        let raw = self.raw.read().await;
        raw.log_messages.clone()
    }

    pub async fn update_expected_ip(&self, expected_ip: AnyIpCidr) {
        let mut raw = self.raw.write().await;
        raw.expected_ip = Option::Some(expected_ip);
        info!(
            "Expected ip of {} set to {}",
            raw.team_number,
            raw.expected_ip.unwrap()
        );
    }

    pub async fn update_mode(&self, mode: Mode) {
        let mut raw = self.raw.write().await;
        raw.mode = mode;
        info!("Mode of {} set to {}", raw.team_number, raw.mode);
    }

    // Internal API -->

    fn new(
        parent: DriverStations,
        team_number: u16,
        alliance_station: AllianceStation,
        initial_mode: Mode, //TODO Fix
    ) -> Self {
        let driverstation = RawDriverStation {
            parent,
            team_number,
            alliance_station,
            mode: initial_mode,
            expected_ip: None,
            active_connection: None,
            confirmed_state: None,
            log_data: None,
            versions: HashMap::new(),
            log_messages: Vec::new(),
        };
        let driverstation = Self {
            raw: Arc::new(RwLock::new(driverstation)),
        };
        driverstation
    }

    pub(super) async fn set_version(&self, version_type: VersionType, version: Version) {
        let mut raw = self.raw.write().await;
        raw.versions.insert(version_type, version);
    }

    pub(super) async fn set_log_data(&self, log_data: Option<LogData>) {
        let mut raw = self.raw.write().await;
        raw.log_data = log_data;
    }

    pub(super) async fn add_log_message(&self, log_message: LogMessage) {
        let mut raw = self.raw.write().await;
        raw.log_messages.push(log_message);
    }

    pub(super) async fn set_confirmed_state(&self, confirmed_state: Option<ConfirmedState>) {
        let mut raw = self.raw.write().await;
        raw.confirmed_state = confirmed_state;
        if raw.active_connection.is_some() {
            raw.active_connection
                .as_ref()
                .unwrap()
                .update_last_udp_packet_reception(Utc::now())
                .await;
        }
    }

    pub(super) async fn set_active_connection(
        &self,
        active_connection: Option<DriverStationConnection>,
    ) {
        let mut raw = self.raw.write().await;
        raw.active_connection = active_connection;
    }
}

//Represents all driverstations (connected and not connected), their sockets, and manages various ways to index them
pub struct RawDriverStations {
    field: Option<Field>,
    all_driverstations: Vec<DriverStation>,
    terminate_signal: Option<broadcast::Sender<()>>,
    running_signal: async_channel::Receiver<()>,
}

#[derive(Clone)]
pub struct DriverStations {
    raw: Arc<RwLock<RawDriverStations>>,
}

impl DriverStations {
    // Public API -->

    pub async fn add_driverstation(
        &mut self,
        team_number: u16,
        alliance_station: AllianceStation,
        initial_mode: Mode, //TODO Fix
    ) -> anyhow::Result<()> {
        if let Some(_) = self
            .get_driverstation_by_team_number(team_number)
            .await
        {
            bail!(
                "Driverstation with team number {} already exists",
                team_number
            );
        }

        if let Some(_) = self
            .get_driverstation_by_position(alliance_station)
            .await
        {
            bail!(
                "Driverstation already exists in alliance station {:?}",
                alliance_station
            );
        }

        let driverstation = DriverStation::new(self.clone(), team_number, alliance_station, initial_mode);

        let mut raw_driverstations = self.raw.write().await;
        raw_driverstations
            .all_driverstations
            .push(driverstation.clone());

        info!(
            "Added driverstation {} to {}",
            driverstation.team_number().await,
            driverstation.alliance_station().await
        );

        Ok(())
    }

    pub async fn delete_driverstation(&mut self, team_number: u16) -> anyhow::Result<()> {
        let raw_driverstations = self.raw.read().await;
        let all_driverstations = raw_driverstations.all_driverstations.clone();
        drop(raw_driverstations);
        let mut new_driverstations: Vec<DriverStation> = Vec::new();

        for ds in all_driverstations.iter() {
            if ds.team_number().await != team_number {
                new_driverstations.push(ds.clone());
            } else {
                let conn = ds.active_connection().await;
                if conn.is_some() {
                    conn.unwrap().kill().await;
                }
            }
        }

        let mut raw_driverstations = self.raw.write().await;
        raw_driverstations.all_driverstations = new_driverstations;
        info!("Deleted driverstation {}", team_number);

        Ok(())
    }

    pub async fn get_driverstation_by_team_number(
        &self,
        team_number: u16,
    ) -> Option<DriverStation> {
        let raw_driverstations = self.raw.read().await;
        let all_driverstations = raw_driverstations.all_driverstations.clone();
        drop(raw_driverstations);
        for ds in all_driverstations.iter() {
            if ds.team_number().await == team_number {
                return Some(ds.clone());
            }
        }
        return None;
    }

    pub async fn get_driverstation_by_position(
        &self,
        alliance_station: AllianceStation,
    ) -> Option<DriverStation> {
        let raw_driverstations = self.raw.read().await;
        let all_driverstations = raw_driverstations.all_driverstations.clone();
        drop(raw_driverstations);
        for ds in all_driverstations.iter() {
            if ds.alliance_station().await == alliance_station {
                return Some(ds.clone());
            }
        }
        return None;
    }

    pub async fn get_field(&self) -> Field {
        let raw_driverstations = self.raw.read().await;
        if let Some(field) = raw_driverstations.field.clone() {
            field
        } else {
            panic!("Driverstations get_field() used too early");
        }
    }

    pub async fn terminate(&self) {
        let mut raw_driverstations = self.raw.write().await;
        drop(raw_driverstations.terminate_signal.take());
    }

    pub async fn wait_for_terminate(&self) {
        let raw_driverstations = self.raw.read().await;
        let running_signal = raw_driverstations.running_signal.clone();
        drop(raw_driverstations);
        let _ = running_signal.recv().await;
    }

    // Internal API -->

    pub(super) fn new(field: Option<Field>) -> Self {
        let (terminate_sender, _) = broadcast::channel(1);

        let (indicate_running, running_signal) = async_channel::bounded(1);

        let driverstations = RawDriverStations {
            field,
            all_driverstations: Vec::new(),
            terminate_signal: Some(terminate_sender),
            running_signal,
        };
        let driverstations = Self {
            raw: Arc::new(RwLock::new(driverstations)),
        };

        let async_driverstations = driverstations.clone();
        tokio::spawn(async move {
            tokio::join!(async_driverstations.tick_loop());
            drop(indicate_running);
        });

        driverstations
    }

    pub(super) async fn set_field(&self, field: Field) -> anyhow::Result<()> {
        let mut raw_driverstations = self.raw.write().await;
        if raw_driverstations.field.is_some() {
            bail!("Field already set");
        }
        raw_driverstations.field = Some(field);
        Ok(())
    }

    async fn tick_loop(&self) {
        let raw_driverstations = self.raw.write().await;
        let mut term_rx = raw_driverstations
            .terminate_signal
            .as_ref()
            .context("Can't start the driverstations tick loop because driverstations has already terminated")
            .unwrap()
            .subscribe();
        drop(raw_driverstations);

        let mut interval = time::interval(Duration::from_millis(250));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    self.tick().await;
                },
                _ = term_rx.recv() => {
                    return
                }
            }
        }
    }

    async fn tick(&self) {
        let raw_driverstations = self.raw.read().await;
        let all_driverstations = raw_driverstations.all_driverstations.clone();
        drop(raw_driverstations);
        for ds in all_driverstations {
            if let Some(conn) = ds.active_connection().await {
                if conn.is_alive().await {
                    if let Err(e) = conn.send_udp_message().await {
                        error!(
                            "Error sending udp message to driver station{}: {}",
                            ds.team_number().await,
                            e
                        );
                    };
                }
            }
        }
    }

    pub(super) async fn decode_udp_message(&self, buffer: Vec<u8>) -> anyhow::Result<()> {
        let mut reader = Cursor::new(buffer);

        let _sequence_num = reader.read_u16().await?; //TODO Track Packet loss
        let _comm_version = reader.read_u8().await?;
        let status_byte = reader.read_u8().await?;
        let team_number = reader.read_u16().await?;
        let battery_byte = reader.read_u16().await?;

        //Status byte info
        let is_emergency_stopped = (status_byte >> 7 & 0x01) == 1;
        let robot_communications_active = (status_byte >> 5 & 0x01) == 1;
        let can_ping_radio = (status_byte >> 4 & 0x01) == 1;
        let can_ping_rio = (status_byte >> 3 & 0x01) == 1;
        let is_enabled = (status_byte >> 2 & 0x01) == 1;
        let mode = Mode::from_byte(status_byte & 0x03);

        let battery_voltage =
            (battery_byte >> 8 & 0xff) as f32 + ((battery_byte & 0xff) as f32 / 256.0);

        let confirmed_state = ConfirmedState {
            is_emergency_stopped,
            robot_communications_active,
            can_ping_radio,
            can_ping_rio,
            is_enabled,
            mode,
            team_number,
            battery_voltage,
        };

        if let Some(ds) = self.get_driverstation_by_team_number(team_number).await {
            ds.set_confirmed_state(Some(confirmed_state)).await;
        } else {
            warn!("Received a packet from a driver station that is not in the list of known driver stations. Team Number: {}", team_number);
        }

        Ok(())
    }

    pub(super) async fn handle_tcp_stream(
        &self,
        tcp_stream: TcpStream,
        ip_address: IpAddr,
    ) -> anyhow::Result<()> {
        tokio::spawn(DriverStationConnection::new(
            tcp_stream,
            ip_address,
            self.get_field().await,
        ));
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub struct ConfirmedState {
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
pub struct LogMessage {
    pub timestamp: u64,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct LogData {
    pub trip_time: u8,
    pub lost_packets: u8,
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
