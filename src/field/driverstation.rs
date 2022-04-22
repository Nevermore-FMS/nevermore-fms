use std::{io::Cursor, net::IpAddr, sync::Arc, time::Duration};

use anyhow::{bail, Context, Ok};
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
    enums::{AllianceStation, Mode}, Field,
};

struct RawDriverstation {
    team_number: u16,
    alliance_station: AllianceStation,
    expected_ip: AnyIpCidr,
    active_connection: Option<DriverStationConnection>,
}

#[derive(Clone)]
pub struct DriverStation {
    raw: Arc<RwLock<RawDriverstation>>,
}

impl DriverStation {
    // Public API -->

    pub fn new(
        team_number: u16,
        alliance_station: AllianceStation,
        expected_ip: AnyIpCidr,
    ) -> Self {
        let driverstation = RawDriverstation {
            team_number,
            alliance_station,
            expected_ip,
            active_connection: None,
        };
        let driverstation = Self {
            raw: Arc::new(RwLock::new(driverstation)),
        };
        driverstation
    }

    pub async fn team_number(&self) -> u16 {
        let raw = self.raw.read().await;
        raw.team_number
    }

    pub async fn alliance_station(&self) -> AllianceStation {
        let raw = self.raw.read().await;
        raw.alliance_station
    }

    pub async fn expected_ip(&self) -> AnyIpCidr {
        let raw = self.raw.read().await;
        raw.expected_ip
    }

    pub async fn active_connection(&self) -> Option<DriverStationConnection> {
        let raw = self.raw.read().await;
        raw.active_connection.clone()
    }

    // Internal API -->

    pub(super) async fn set_confirmed_state(&self, confirmed_state: ConfirmedState) {
        let mut raw = self.raw.write().await;
        raw.team_number = confirmed_state.team_number;
    }

    pub(super) async fn set_active_connection(&self, active_connection: DriverStationConnection) {
        let mut raw = self.raw.write().await;
        raw.active_connection = Some(active_connection);
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

    pub async fn add_driverstation(&mut self, driverstation: DriverStation) -> anyhow::Result<()> {
        if let Some(_) = self
            .get_driverstation_by_team_number(driverstation.team_number().await)
            .await
        {
            bail!(
                "Driverstation with team number {} already exists",
                driverstation.team_number().await
            );
        }

        if let Some(_) = self.get_driverstation_by_position(driverstation.alliance_station().await).await {
            bail!(
                "Driverstation already exists in alliance station {:?}",
                driverstation.alliance_station().await
            );
        }

        let mut raw_driverstations = self.raw.write().await;
        raw_driverstations.all_driverstations.push(driverstation);

        Ok(())
    }

    pub async fn get_driverstation_by_team_number(
        &self,
        team_number: u16,
    ) -> Option<DriverStation> {
        let raw_driverstations = self.raw.read().await;
        for ds in raw_driverstations.all_driverstations.iter() {
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
        for ds in raw_driverstations.all_driverstations.iter() {
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

        let mut interval = time::interval(Duration::from_millis(500));
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
        for ds in raw_driverstations.all_driverstations.iter() {
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
        let mode = Mode::from_integer(status_byte & 0x03);

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
            ds.set_confirmed_state(confirmed_state).await;
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
            self.clone(),
        ));
        Ok(())
    }
}

#[derive(Clone, Copy)]
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
