use std::{
    convert::TryInto,
    io::Cursor,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use anyhow::Context;
use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use log::*;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        TcpStream, UdpSocket,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::RwLock,
};
use tokio_util::sync::CancellationToken;

use super::{
    Field,
    driverstation::{DriverStation, DriverStationLogData, DriverStationLogMessage},
    enums::{AllianceStation, DriverstationStatus, Mode, VersionData, VersionType},
};

struct RawDriverStationConnection {
    uuid: uuid::Uuid,
    field: Field,
    /// Represents the current driver station, only if this station is expected to be connected
    parent: Option<DriverStation>,
    tcp_writer: Option<OwnedWriteHalf>,
    udp_socket: Option<Arc<UdpSocket>>,
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
        raw.tcp_writer.is_some()
    }

    pub async fn uuid(&self) -> uuid::Uuid {
        let raw = self.raw.read().await;
        raw.uuid.clone()
    }

    pub async fn field(&self) -> Field {
        let raw = self.raw.read().await;
        raw.field.clone()
    }

    /// Represents the current driver station, only if this station is expected to be connected
    pub async fn parent(&self) -> Option<DriverStation> {
        let raw = self.raw.read().await;
        raw.parent.clone()
    }

    pub async fn ip_address(&self) -> IpAddr {
        let raw = self.raw.read().await;
        raw.ip_address
    }

    pub async fn last_udp_packet_reception(&self) -> DateTime<Utc> {
        let raw = self.raw.read().await;
        raw.last_udp_packet_reception
    }

    pub async fn kill(&self) {
        let mut raw = self.raw.write().await;

        if let Some(mut tcp_writer) = raw.tcp_writer.take() {
            if let Some(ds) = &raw.parent {
                ds.remove_active_connection().await;
                ds.set_confirmed_state(None).await;
                info!(
                    "Driver station {} disconnected (Conn ID: {})",
                    ds.team_number().await,
                    raw.uuid
                );
            } else {
                info!(
                    "Driver station connection disconnected (Conn ID: {})",
                    raw.uuid
                );
            }
            if let Err(e) = tcp_writer.shutdown().await {
                error!("Failed to shutdown TCP stream: {}", e);
            }
        }
    }

    // Internal API -->
    pub(super) fn new(ip_address: IpAddr, field: Field) -> Self {
        let driver_station_connection = RawDriverStationConnection {
            field,
            parent: None,
            tcp_writer: None,
            udp_socket: None,
            ip_address,
            udp_outgoing_sequence_num: 0,
            last_udp_packet_reception: Utc::now(),
            uuid: uuid::Uuid::new_v4(),
        };
        let driver_station_connection = Self {
            raw: Arc::new(RwLock::new(driver_station_connection)),
        };

        driver_station_connection
    }

    pub(super) async fn run(
        self,
        tcp_stream: TcpStream,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
        let (owned_read_half, owned_write_half) = tcp_stream.into_split();
        let mut raw = self.raw.write().await;
        raw.tcp_writer = Some(owned_write_half);
        raw.udp_socket = Some(udp_socket);
        drop(raw);

        let task_handle = tokio::task::Builder::new()
            .name("DriverStationConnection TCP Stream Handler")
            .spawn(async move {
                if let Err(e) = self
                    .handle_tcp_stream(owned_read_half, cancellation_token)
                    .await
                {
                    warn!("Error handling TCP stream from driverstation: {}", e);
                    self.kill().await
                }
            })?;

        task_handle.await?;

        Ok(())
    }

    pub(super) async fn update_last_udp_packet_reception(&self, time: DateTime<Utc>) {
        let mut raw = self.raw.write().await;
        raw.last_udp_packet_reception = time;
    }

    async fn set_parent(&self, parent: Option<DriverStation>) {
        let mut raw = self.raw.write().await;
        raw.parent = parent;
    }

    async fn handle_tcp_stream(
        &self,
        mut tcp_reader: OwnedReadHalf,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut read_stream = async || -> anyhow::Result<()> {
            loop {
                let mut buffer = [0; 2];
                tcp_reader.read_exact(&mut buffer).await?;
                let mut reader = Cursor::new(buffer);
                let packet_length = reader.read_u16().await?;

                let mut buffer = vec![0; packet_length as usize];
                tcp_reader.read_exact(&mut buffer).await?;
                let mut reader = Cursor::new(buffer);
                let id = reader.read_u8().await?;

                match id {
                    0x18 => {
                        // Team Number packet
                        let team_number = reader.read_u16().await?;
                        if let Some(ds) = self
                            .field()
                            .await
                            .driverstations()
                            .await
                            .get_driverstation_by_team_number(team_number)
                            .await
                        {
                            self.set_parent(Some(ds.clone())).await;
                            if let Some(old_conn) = ds.remove_active_connection().await {
                                old_conn.kill().await
                            }
                            ds.set_active_connection(self.clone()).await;
                            info!("Driver station {} connected", team_number);
                        } else {
                            warn!(
                                "Received a connection from a driver station that is not in the list of known driver stations. Team Number: {}",
                                team_number
                            );
                        }

                        self.send_tcp_station_info().await?;
                        self.send_tcp_event_code().await?;
                    }
                    0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 => {
                        // Version Codes
                        //TODO Maybe use regex?
                        let mut version_unparsed = String::new();
                        reader.read_to_string(&mut version_unparsed).await.ok();
                        let split: Vec<&str> = version_unparsed.split(">").collect();
                        let (status, version) = if split.len() > 1 {
                            (
                                split[0].trim_start_matches("<").to_string(),
                                split[1].to_string(),
                            )
                        } else {
                            (String::new(), String::new())
                        };

                        let version_type = VersionType::from_byte(id).unwrap();
                        let version = VersionData {
                            version_type,
                            status,
                            version,
                        };

                        let raw_conn = self.raw.read().await;
                        let ds = raw_conn.parent.clone();
                        drop(raw_conn);
                        if ds.is_some() {
                            ds.unwrap().set_version(version_type, version).await;
                        }
                    }
                    0x16 => {
                        // Log Data Packet
                        let timestamp = Utc::now().timestamp() as u64;

                        let trip_time = reader.read_u8().await? / 2;
                        let lost_packets = reader.read_u8().await?;

                        let voltage_byte = reader.read_u16().await?;
                        let voltage = (voltage_byte >> 8 & 0xff) as f32
                            + ((voltage_byte & 0xff) as f32 / 256.0);

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
                        let ds = raw_conn.parent.clone();
                        drop(raw_conn);
                        if ds.is_some() {
                            ds.unwrap()
                                .record_log_data(DriverStationLogData {
                                    timestamp,
                                    trip_time,
                                    lost_packets,
                                    voltage,
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
                                    bandwidth,
                                })
                                .await;
                        }
                    }
                    0x17 => {
                        // Log Message Packet
                        let timestamp = Utc::now().timestamp() as u64;
                        let _ = reader.read_u32().await?; // Message Count (Seems to always be 1?) - Chase
                        let local_timestamp = reader.read_u64().await? - 2082844800; // Offset from LabView epoch to UNIX Epoch
                        reader.read_u64().await?;
                        let mut data = String::new();
                        reader.read_u32().await?;
                        reader.read_to_string(&mut data).await.ok();

                        let raw_conn = self.raw.read().await;
                        let ds = raw_conn.parent.clone();
                        drop(raw_conn);
                        if ds.is_some() {
                            ds.unwrap()
                                .add_log_message(DriverStationLogMessage {
                                    timestamp,
                                    local_timestamp,
                                    message: data,
                                })
                                .await;
                        }
                    }
                    0x1d => { /* Keep-Alive Packet, doesn't need a reply */ }
                    unknown_id => {
                        warn!(
                            "Received a TCP packet from a driverstation with an unknown id {:#x} and size {}",
                            unknown_id, packet_length
                        );
                    }
                }
            }
        };

        let died = async {
            while self.is_alive().await {
                tokio::task::yield_now().await;
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => {
                self.kill().await;
                Ok(())
            },
            _ = died => Ok(()),
            res = read_stream() => res.context("TCP Stream handler closed unexpectedly"),
        }
    }

    async fn send_tcp_station_info(&self) -> anyhow::Result<()> {
        let mut alliance_station = AllianceStation::None;
        let mut status = DriverstationStatus::Waiting;

        if let Some(ds) = self.parent().await {
            alliance_station = ds.alliance_station().await;
            status = DriverstationStatus::Good;

            if let Some(expected_ip) = ds.expected_ip().await {
                if !expected_ip.contains(&self.ip_address().await) {
                    status = DriverstationStatus::Bad;
                    info!(
                        "Driver station {} is not expected to be connected from this IP address ({})",
                        ds.team_number().await,
                        self.ip_address().await
                    );
                }
            }
        }

        let mut packet = Cursor::new(Vec::new());
        packet.write_u8(0x19).await?; //0x19 = ID For Station Info
        packet.write_u8(alliance_station.to_byte()).await?;
        packet.write_u8(status.to_byte()).await?;
        let buffer = packet.into_inner();

        let mut outer_packet = Cursor::new(Vec::<u8>::new());
        outer_packet
            .write_u16(buffer.len().try_into().unwrap())
            .await?;
        outer_packet.write_all(&buffer).await?;

        let mut raw_conn = self.raw.write().await;
        let Some(ref mut tcp_writer) = raw_conn.tcp_writer else {
            anyhow::bail!("This DriverStationConnection is already closed")
        };
        tcp_writer.write_all(&outer_packet.into_inner()).await?;

        Ok(())
    }

    async fn send_tcp_event_code(&self) -> anyhow::Result<()> {
        let raw_conn = self.raw.read().await;

        let event_code = raw_conn.field.event_name().await;

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
        let Some(ref mut tcp_writer) = raw_conn.tcp_writer else {
            anyhow::bail!("This DriverStationConnection is already closed")
        };
        tcp_writer.write_all(&outer_packet.into_inner()).await?;

        Ok(())
    }

    pub(super) async fn send_udp_message(&self) -> anyhow::Result<()> {
        let Some(ds) = self.parent().await else {
            anyhow::bail!(
                "This DriverStationConnection does not have a parent DriverStation assigned"
            )
        };

        let mut raw_conn = self.raw.write().await;
        let Some(udp_socket) = raw_conn.udp_socket.clone() else {
            anyhow::bail!("This DriverStationConnection does not have a UdpSocket")
        };

        if raw_conn.udp_outgoing_sequence_num >= u16::max_value() {
            raw_conn.udp_outgoing_sequence_num = 0;
        } else {
            raw_conn.udp_outgoing_sequence_num += 1;
        }

        let seq_num = raw_conn.udp_outgoing_sequence_num;

        drop(raw_conn);

        let mut packet = Cursor::new(Vec::new());
        packet.write_u16(seq_num).await?;
        packet.write_u8(0x00).await?; //Comm Version

        let driverstations = self.field().await.driverstations().await;
        let field = driverstations.get_field().await;
        let ip_address = self.ip_address().await;
        

        let mut control_byte = 0x00;
        match field.ds_mode().await {
            Mode::TeleOp => control_byte |= 0x00,
            Mode::Test => control_byte |= 0x01,
            Mode::Autonomous => control_byte |= 0x02,
        }

        if ds.enabled().await {
            control_byte |= 0x04
        }

        if field
            .alarm_handler()
            .await
            .is_target_faulted(field.alarm_target().await.as_str())
            .await
        {
            // EStop DS if field is faulted
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
            .write_u16(driverstations.get_field().await.match_number().await)
            .await?; //Match Number
        packet
            .write_u8(driverstations.get_field().await.play_number().await)
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

        Ok(())
    }
}
