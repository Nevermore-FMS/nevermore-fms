use std::{sync::Arc, io::Cursor, convert::TryInto, net::{IpAddr, SocketAddr}};

use chrono::{Local, Timelike, Datelike};
use log::*;
use tokio::{sync::{Mutex, RwLock}, net::{TcpStream, UdpSocket}, io::{AsyncReadExt, AsyncWriteExt}};

use super::{enums::{AllianceStation, DriverstationStatus}, driverstation::{DriverStations, DriverStation}};

struct RawDriverStationConnection {
    driverstations: DriverStations, /// Represents the list of known/expected driver stations
    driverstation: Option<DriverStation>, /// Represents the current driver station, only if this station is expected to be connected
    alive: bool,
    tcp_stream: Arc<Mutex<TcpStream>>,
    ip_address: IpAddr,
    udp_outgoing_sequence_num: u16
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
        raw.alive
    }

    pub async fn kill(&self) {
        let mut raw = self.raw.write().await;
        raw.alive = false;
        let mut tcp_stream = raw.tcp_stream.lock().await;
        if let Err(e) = tcp_stream.shutdown().await {
            error!("Failed to shutdown TCP stream: {}", e);
        }
    }

    // Internal API -->
    pub(super) async fn new(tcp_stream: TcpStream, ip_address: IpAddr, driverstations: DriverStations) -> Self {
        let driver_station_connection = RawDriverStationConnection {
            driverstations,
            driverstation: None,
            alive: true,
            tcp_stream: Arc::new(Mutex::new(tcp_stream)),
            ip_address,
            udp_outgoing_sequence_num: 0
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

        driver_station_connection
    }

    async fn handle_tcp_stream(&self) -> anyhow::Result<()> {
        loop {
            let raw_conn = self.raw.read().await;
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
                        ds.set_active_connection(self.clone()).await;
                        info!("Driver station {} connected", team_number);
                    } else {
                        warn!("Received a connection from a driver station that is not in the list of known driver stations. Team Number: {}", team_number);
                    }

                    drop(raw_conn);
                    drop(tcp_stream);
                    self.send_tcp_station_info().await?;
                }
                unknown_id => {
                    warn!(
                        "Received a TCP packet from a driverstation with an unknown id {:#x}",
                        unknown_id
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
            if ds.expected_ip().await.contains(&raw_conn.ip_address) {
                status = DriverstationStatus::Good;
            } else {
                status = DriverstationStatus::Bad;
                info!("Driver station {} is not expected to be connected from this IP address", ds.team_number().await);
            }
        }

        let mut packet = Cursor::new(Vec::new());
        packet.write_u8(0x19).await?; //ID For Station Info
        packet.write_u8(alliance_station.to_byte()).await?;
        packet.write_u8(status.to_byte()).await?; //Station Status TODO
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
            packet.write_u8(0x00).await?; //Control Byte TODO
            packet.write_u8(0x00).await?; //Request Byte
            packet.write_u8(ds.alliance_station().await.to_byte()).await?; //Alliance Station
            packet.write_u8(0x00).await?; //Tournament Level TODO
            packet.write_u16(0).await?; //Match Number TODO
            packet.write_u8(0x00).await?; //Play Number TODO
            
            let time = Local::now();
            packet.write_u32(time.nanosecond()/1000).await?;
            packet.write_u8(time.second().try_into().unwrap()).await?;
            packet.write_u8(time.minute().try_into().unwrap()).await?;
            packet.write_u8(time.hour().try_into().unwrap()).await?;
            packet.write_u8(time.day().try_into().unwrap()).await?;
            packet.write_u8(time.month0().try_into().unwrap()).await?;
            packet.write_u8((time.year() - 1900).try_into().unwrap()).await?;

            packet.write_u16(100).await?; //Time Remaining TODO
    
            let buffer = packet.into_inner();

            let socket = UdpSocket::bind("0.0.0.0:0").await?;
            socket.send_to(&buffer, SocketAddr::from((raw_conn.ip_address, 1121))).await?;
        }

        Ok(())
    }
}

