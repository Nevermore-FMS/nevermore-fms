use std::{
    convert::TryInto,
    io::Cursor,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use chrono::{Datelike, Local, Timelike, DateTime, Utc};
use log::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, UdpSocket},
    sync::{Mutex, RwLock},
};

use crate::plugin::rpc;

use super::{
    driverstation::{DriverStation, DriverStations},
    enums::{AllianceStation, DriverstationStatus, Mode, VersionType, Version},
};

struct RawDriverStationConnection {
    driverstations: DriverStations,
    /// Represents the list of known/expected driver stations
    driverstation: Option<DriverStation>,
    /// Represents the current driver station, only if this station is expected to be connected
    alive: bool,
    tcp_stream: Arc<Mutex<TcpStream>>,
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

    pub async fn kill(&self) {
        let mut raw = self.raw.write().await;
        raw.alive = false;
        if let Some(ds) = &raw.driverstation {
            ds.set_active_connection(None).await;
            ds.set_confirmed_state(None).await;
        }
        let mut tcp_stream = raw.tcp_stream.lock().await;
        if let Err(e) = tcp_stream.shutdown().await {
            error!("Failed to shutdown TCP stream: {}", e);
        }
        drop(tcp_stream);
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
        let driver_station_connection = RawDriverStationConnection {
            driverstations,
            driverstation: None,
            alive: true,
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
            udp_socket: Arc::new(UdpSocket::bind("0.0.0.0:0").await?),
            ip_address,
            udp_outgoing_sequence_num: 0,
            last_udp_packet_reception: Utc::now()
        };
        let driver_station_connection = Self {
            raw: Arc::new(RwLock::new(driver_station_connection)),
        };

        let conn = driver_station_connection.clone();
        tokio::spawn(async move {
            if let Err(e) = conn.handle_tcp_stream().await {
                if e.to_string() == "early eof" {
                    let raw_conn = conn.raw.read().await;
                    if let Some(ds) = &raw_conn.driverstation {
                        info!("Driver station {} disconnected", ds.team_number().await);
                    }
                } else {
                    warn!("Error handling TCP stream from driverstation: {}", e);
                }
                let mut raw_conn = conn.raw.write().await;
                raw_conn.alive = false;
            }
        });

        Ok(driver_station_connection)
    }

    async fn handle_tcp_stream<'a>(&self) -> anyhow::Result<()> {
        loop {
            let raw_conn = self.raw.read().await;
            if !raw_conn.alive {
                return Ok(());
            };
            let tcp_stream = raw_conn.tcp_stream.clone();
            drop(raw_conn);
            let mut tcp_stream = tcp_stream.lock().await;

            let mut buffer = [0; 2];
            tcp_stream.read_exact(&mut buffer).await?;
            let mut reader = Cursor::new(buffer);
            let packet_length = reader.read_u16().await?;

            let mut buffer = vec![0; packet_length as usize];
            tcp_stream.read_exact(&mut buffer).await?;
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

                    drop(tcp_stream);
                    self.send_tcp_station_info().await?;
                    self.send_tcp_event_code().await?;
                },
                0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => {
                    let mut version_unparsed = String::new();
                    reader.read_to_string(&mut version_unparsed).await?;
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
                0x17 => {
                    let count = reader.read_u32().await?;
                    let timestamp = reader.read_u64().await?;
                    reader.read_u64().await?;
                    let mut data = String::new();
                    reader.read_u32().await?;
                    reader.read_to_string(&mut data).await?;
                    println!("{} {} {}", count, timestamp, data);
                }
                0x16 => {

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
        let mut tcp_stream = raw_conn.tcp_stream.lock().await;

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

        tcp_stream.write_all(&outer_packet.into_inner()).await?;

        Ok(())
    }

    async fn send_tcp_event_code(&self) -> anyhow::Result<()> {
        let raw_conn = self.raw.read().await;
        let mut tcp_stream = raw_conn.tcp_stream.lock().await;

        let event_code = raw_conn.driverstations.get_field().await.event_name().await;

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

        tcp_stream.write_all(&outer_packet.into_inner()).await?;

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
