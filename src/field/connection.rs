use std::{
    convert::TryInto,
    io::{Cursor, BufRead},
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use chrono::{Datelike, Local, Timelike, DateTime, Utc};
use log::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt},
    net::{TcpStream, UdpSocket, tcp::{OwnedReadHalf, OwnedWriteHalf}},
    sync::{Mutex, RwLock},
};

use crate::plugin::rpc::{self, LogMessage};

use super::{
    driverstation::{DriverStation, DriverStations},
    enums::{AllianceStation, DriverstationStatus, Mode, VersionType, Version, LogData},
};

struct RawDriverStationConnection {
    uuid: uuid::Uuid,
    driverstations: DriverStations,
    /// Represents the list of known/expected driver stations
    driverstation: Option<DriverStation>,
    /// Represents the current driver station, only if this station is expected to be connected
    alive: bool,
    writer: OwnedWriteHalf,
    udp_socket: Arc<UdpSocket>,
    ip_address: IpAddr,
    udp_outgoing_sequence_num: u16,
    last_udp_packet_reception: DateTime<Utc>,
}

/// Represents the long-lived connection to the driver station
#[derive(Clone)]
pub struct DriverStationConnection {
    raw: Arc<RwLock<RawDriverStationConnection>>,
}

impl DriverStationConnection {
    //Public API -->

    pub async fn is_alive(&self) -> bool {
        let raw = self.raw.read().await;
        if raw.alive && Utc::now().signed_duration_since(raw.last_udp_packet_reception) > chrono::Duration::seconds(2) {
            drop(raw);
            self.kill().await;
            return false
        }
        
        raw.alive
    }

    pub async fn update_last_udp_packet_reception(&self, time: DateTime<Utc>) {
        let mut raw = self.raw.write().await;
        raw.last_udp_packet_reception = time;
    }

    pub async fn uuid(&self) -> uuid::Uuid {
        let raw = self.raw.read().await;
        raw.uuid.clone()
    }

    pub async fn kill(&self) {
        let mut raw = self.raw.write().await;
        raw.alive = false;
        if let Some(ds) = &raw.driverstation {
            ds.set_active_connection(None).await;
            ds.set_confirmed_state(None).await;
        }
        if let Err(e) = raw.writer.shutdown().await {
            error!("Failed to shutdown TCP stream: {}", e);
        }
        drop(raw);
    }

    pub async fn to_rpc(&self) -> rpc::DriverStationConnection {
        let raw = self.raw.read().await;

        rpc::DriverStationConnection{
            alive: raw.alive,
            ip: raw.ip_address.to_string(),
            outgoing_sequence_num: raw.udp_outgoing_sequence_num as u32,
        }
    }

    // Internal API -->
    pub(super) async fn new(
        tcp_stream: TcpStream,
        ip_address: IpAddr,
        driverstations: DriverStations,
    ) -> anyhow::Result<Self> {
        let (owned_read_half, owned_write_half) = tcp_stream.into_split();
        let driver_station_connection = RawDriverStationConnection {
            driverstations,
            driverstation: None,
            alive: true,
            writer: owned_write_half,
            udp_socket: Arc::new(UdpSocket::bind("0.0.0.0:0").await?),
            ip_address,
            udp_outgoing_sequence_num: 0,
            last_udp_packet_reception: Utc::now(),
            uuid: uuid::Uuid::new_v4()
        };
        let driver_station_connection = Self {
            raw: Arc::new(RwLock::new(driver_station_connection)),
        };

        let conn = driver_station_connection.clone();
        tokio::spawn(async move {
            if let Err(e) = conn.handle_tcp_stream(owned_read_half).await {
                if e.to_string() == "early eof" {
                    let raw_conn = conn.raw.read().await;
                    if let Some(ds) = &raw_conn.driverstation {
                        info!("Driver station {} disconnected", ds.team_number().await);
                    }
                    drop(raw_conn);
                } else {
                    warn!("Error handling TCP stream from driverstation: {}", e);
                }
                let mut raw_conn = conn.raw.write().await;
                raw_conn.alive = false;
            }
        });

        Ok(driver_station_connection)
    }

    async fn handle_tcp_stream(&self, mut tcp: OwnedReadHalf) -> anyhow::Result<()> {
        loop {
            let raw_conn = self.raw.read().await;
            if !raw_conn.alive {
                return Ok(());
            };
            drop(raw_conn);

            let mut buffer = [0; 2];
            tcp.read_exact(&mut buffer).await?;
            let mut reader = Cursor::new(buffer);
            let packet_length = reader.read_u16().await?;        

            let mut buffer = vec![0; packet_length as usize];
            tcp.read_exact(&mut buffer).await?;
            let mut reader = Cursor::new(buffer);
            let id = reader.read_u8().await?;

            match id {
                0x18 => {
                    let team_number = reader.read_u16().await?;
                    let mut raw_conn = self.raw.write().await;
                    if let Some(ds) = raw_conn
                        .driverstations
                        .get_driverstation_by_team_number(team_number)
                        .await
                    {
                        raw_conn.driverstation = Some(ds.clone());
                        drop(raw_conn);
                        ds.set_active_connection(Some(self.clone())).await;
                        info!("Driver station {} connected", team_number);
                    } else {
                        warn!("Received a connection from a driver station that is not in the list of known driver stations. Team Number: {}", team_number);
                        drop(raw_conn);
                    }

                    self.send_tcp_station_info().await?;
                    self.send_tcp_event_code().await?;
                },
                0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => { // Version Codes
                    let mut version_unparsed = String::new();
                    reader.read_to_string(&mut version_unparsed).await.ok();
                    let split: Vec<&str> = version_unparsed.split(">").collect();
                    let (status, version) = if split.len() > 1 {
                        (split[0].trim_start_matches("<").to_string(), split[1].to_string())
                    } else {
                        (String::new(), String::new())
                    };
                    
                    let version_type = VersionType::from_byte(id);
                    let version = Version{
                        status,
                        version
                    };

                    let raw_conn = self.raw.read().await;
                    let ds = raw_conn.driverstation.clone();
                    drop(raw_conn);
                    if ds.is_some() {
                        ds.unwrap().set_version(version_type, version).await;
                    }
                }
                0x17 => { // Log Message Packet
                    let _ = reader.read_u32().await?; // Message Count (Seems to always be 1?) - Chase
                    let timestamp = reader.read_u64().await?;
                    reader.read_u64().await?;
                    let mut data = String::new();
                    reader.read_u32().await?;
                    reader.read_to_string(&mut data).await.ok();

                    let raw_conn = self.raw.read().await;
                    let ds = raw_conn.driverstation.clone();
                    drop(raw_conn);
                    if ds.is_some() {
                        ds.unwrap().add_log_message(LogMessage{
                            timestamp,
                            message: data
                        }).await;
                    }
                }
                0x16 => { // Log Data Packet
                    let trip_time = reader.read_u8().await? / 2;
                    let lost_packets = reader.read_u8().await?;

                    // This packet technically includes Robot Battery Voltage, but ConfirmedState already returns this.
                    reader.read_u16().await?;

                    let status_byte = reader.read_u8().await?;
                    let brownout = (status_byte >> 7 & 0x01) == 1;
                    let watchdog = (status_byte >> 6 & 0x01) == 1;
                    let ds_teleop = (status_byte >> 5 & 0x01) == 1;
                    let ds_auto = (status_byte >> 4 & 0x01) == 1;
                    let ds_disable = (status_byte >> 3 & 0x01) == 1;
                    let robot_teleop = (status_byte >> 2 & 0x01) == 1;
                    let robot_auto = (status_byte >> 1 & 0x01) == 1;
                    let robot_disable = (status_byte >> 0 & 0x01) == 1;

                    let can_utilization = reader.read_u8().await? / 2;
                    let signal = reader.read_u8().await? / 2;
                    let bandwidth = reader.read_u16().await? as f32 / 256.0;

                    let raw_conn = self.raw.read().await;
                    let ds = raw_conn.driverstation.clone();
                    drop(raw_conn);
                    if ds.is_some() {
                        ds.unwrap().set_log_data(Some(LogData{
                            trip_time,
                            lost_packets,
                            brownout,
                            watchdog,
                            ds_teleop,
                            ds_auto,
                            ds_disable,
                            robot_teleop,
                            robot_auto,
                            robot_disable,
                            can_utilization,
                            signal,
                            bandwidth
                        })).await;
                    }
                }
                0x1d => {/* Keep-Alive Packet, doesn't need a reply */}
                unknown_id => {
                    warn!(
                        "Received a TCP packet from a driverstation with an unknown id {:#x} and size {}",
                        unknown_id, packet_length
                    );
                }
            }
        }
    }

    async fn send_tcp_station_info(&self) -> anyhow::Result<()> {
        let raw_conn = self.raw.read().await;

        let mut alliance_station = AllianceStation::None;
        let mut status = DriverstationStatus::Waiting;

        if let Some(ds) = raw_conn.driverstation.clone() {
            alliance_station = ds.alliance_station().await;
            if let Some(expected_ip) = ds.expected_ip().await.take() {
                if expected_ip.contains(&raw_conn.ip_address) {
                    status = DriverstationStatus::Good;
                } else {
                    status = DriverstationStatus::Bad;
                    info!(
                        "Driver station {} is not expected to be connected from this IP address ({})",
                        ds.team_number().await,
                        &raw_conn.ip_address
                    );
                }
            } else {
                status = DriverstationStatus::Waiting;
            }
        }

        drop(raw_conn);


        let mut packet = Cursor::new(Vec::new());
        packet.write_u8(0x19).await?; //ID For Station Info
        packet.write_u8(alliance_station.to_byte()).await?;
        packet.write_u8(status.to_byte()).await?;
        let buffer = packet.into_inner();

        let mut outer_packet = Cursor::new(Vec::<u8>::new());
        outer_packet
            .write_u16(buffer.len().try_into().unwrap())
            .await?;
        outer_packet.write_all(&buffer).await?;

        let mut raw_conn = self.raw.write().await;

        raw_conn.writer.write_all(&outer_packet.into_inner()).await?;

        Ok(())
    }

    async fn send_tcp_event_code(&self) -> anyhow::Result<()> {
        let raw_conn = self.raw.read().await;

        let event_code = raw_conn.driverstations.get_field().await.event_name().await;

        drop(raw_conn);

        let mut packet = Cursor::new(Vec::new());
        packet.write_u8(0x14).await?; //ID For Event Code
        packet.write_u8(event_code.len() as u8).await?;
        packet.write(event_code.as_bytes()).await?;
        let buffer = packet.into_inner();

        let mut outer_packet = Cursor::new(Vec::<u8>::new());
        outer_packet
            .write_u16(buffer.len().try_into().unwrap())
            .await?;
        outer_packet.write_all(&buffer).await?;

        let mut raw_conn = self.raw.write().await;

        raw_conn.writer.write_all(&outer_packet.into_inner()).await?;

        Ok(())
    }

    pub(super) async fn send_udp_message(&self) -> anyhow::Result<()> {
        let mut raw_conn = self.raw.write().await;

        if let Some(ds) = raw_conn.driverstation.clone() {
            if raw_conn.udp_outgoing_sequence_num >= u16::max_value() {
                raw_conn.udp_outgoing_sequence_num = 0;
            } else {
                raw_conn.udp_outgoing_sequence_num += 1;
            }

            let mut packet = Cursor::new(Vec::new());
            packet.write_u16(raw_conn.udp_outgoing_sequence_num).await?;
            packet.write_u8(0x00).await?; //Comm Version

            let driverstations = raw_conn.driverstations.clone();
            let udp_socket = raw_conn.udp_socket.clone();
            let ip_address = raw_conn.ip_address.clone();
            drop(raw_conn);

            let mut control_byte = 0x00;
            match ds.mode().await {
                Mode::TeleOp => control_byte |= 0x00,
                Mode::Test => control_byte |= 0x01,
                Mode::Autonomous => control_byte |= 0x02,
            }

            if driverstations
                .get_field()
                .await
                .control_system()
                .await
                .is_ds_enabled(ds.clone())
                .await
            {
                control_byte |= 0x04
            }

            if driverstations
                .get_field()
                .await
                .control_system()
                .await
                .is_ds_estopped(ds.clone())
                .await
            {
                control_byte |= 0x80
            }

            packet.write_u8(control_byte).await?;
            packet.write_u8(0x00).await?; //Request Byte
            packet
                .write_u8(ds.alliance_station().await.to_byte())
                .await?; //Alliance Station
            packet
                .write_u8(
                    driverstations
                        .get_field()
                        .await
                        .tournament_level()
                        .await
                        .to_byte(),
                )
                .await?; //Tournament Level
            packet
                .write_u16(
                    driverstations
                        .get_field()
                        .await
                        .match_number()
                        .await,
                )
                .await?; //Match Number
            packet
                .write_u8(
                    driverstations
                        .get_field()
                        .await
                        .play_number()
                        .await,
                )
                .await?; //Play Number

            let time = Local::now();
            packet.write_u32(time.nanosecond() / 1000).await?;
            packet.write_u8(time.second().try_into().unwrap()).await?;
            packet.write_u8(time.minute().try_into().unwrap()).await?;
            packet.write_u8(time.hour().try_into().unwrap()).await?;
            packet.write_u8(time.day().try_into().unwrap()).await?;
            packet.write_u8(time.month().try_into().unwrap()).await?;
            packet
                .write_u8((time.year() - 1900).try_into().unwrap())
                .await?;

            packet
                .write_u16(
                    driverstations
                        .get_field()
                        .await
                        .timer()
                        .await
                        .current_time_remaining()
                        .as_secs() as u16,
                )
                .await?; //Time Remaining

            let buffer = packet.into_inner();

            udp_socket
                .send_to(&buffer, SocketAddr::from((ip_address, 1121)))
                .await?;
        }

        Ok(())
    }
}
