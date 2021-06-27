use chrono::{Datelike, Local, Timelike};

use serde::Serialize;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use crate::field::{ThreadSafeRobotMap, ThreadSafeAllianceStationMap};
use crate::field::enums::{Mode, AllianceStation, DriverstationStatus};
use std::borrow::BorrowMut;

pub type ThreadSafeRobot = Arc<Mutex<Robot>>;

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub emergency_stop: bool,
    pub enable: bool,

    pub mode: Mode,

    pub team_number: u16,
    pub alliance_station: AllianceStation,
    pub status: DriverstationStatus,
    pub sequence_number: u16,
    pub time_to_display: u16,
    pub match_number: u16,
    pub event_name: String,
}

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldOverride {
    pub emergency_stop: bool,
    pub disabled: bool,
}

pub struct Robot {
    confirmed_state: Option<ConfirmedState>,
    state: Option<State>,
    field_override: FieldOverride,
    tcp_writer: OwnedWriteHalf,
    udp_socket: Arc<UdpSocket>,
    socket_address: SocketAddr,
    robot_map: ThreadSafeRobotMap,
    alliance_station_map: ThreadSafeAllianceStationMap,
    closing_sender: Sender<()>,
    original_event_name: String,
    has_closed: bool
}

impl Robot {
    pub fn override_emergency_stop(&mut self, emergency_stop: bool) {
        self.field_override.emergency_stop = emergency_stop;
    }

    pub fn override_enabled(&mut self, enabled: bool) {
        self.field_override.disabled = !enabled;
    }

    pub fn get_confirmed_state(&self) -> anyhow::Result<ConfirmedState> {
        self.confirmed_state.ok_or(anyhow::anyhow!("confirmed_state hasn't been formed yet"))
    }

    pub fn get_state(&self) -> anyhow::Result<State> {
        Ok(self.state.clone().ok_or(anyhow::anyhow!("state hasn't been formed yet"))?.clone())
    }

    pub fn set_state(&mut self, state: State) {
        if self.state.is_some() {
            let old_state = self.state.clone().unwrap();
            if old_state.event_name != state.event_name ||
                old_state.alliance_station != state.alliance_station ||
                old_state.status != state.status {
                self.send_event_name();
                self.send_station_info();
            }
        }
        self.state = Some(state);
    }

    pub fn address(&self) -> SocketAddr {
        self.socket_address
    }

    pub fn has_closed(&self) -> bool {
        self.has_closed
    }

    // Internal API -->

    // Remember everything is big endian with the FMS because they like suffering.
    pub async fn handle_connection(
        socket: TcpStream,
        socket_address: SocketAddr,
        robot_map: ThreadSafeRobotMap,
        alliance_station_map: ThreadSafeAllianceStationMap,
        udp_socket: Arc<UdpSocket>,
        event_name: String
    ) {
        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
            let (mut reader, writer) = socket.into_split();
            let robot = Robot {
                confirmed_state: None,
                state: None,
                field_override: FieldOverride { emergency_stop: false, disabled: false },
                tcp_writer: writer,
                udp_socket,
                socket_address,
                robot_map,
                alliance_station_map,
                closing_sender: tx,
                original_event_name: event_name,
                has_closed: false
            };

            let mut thread_safe_robot = Arc::new(Mutex::new(robot));

            let mut buffer: Vec<u8> = vec![0; 50];

            loop {
                tokio::select! {
                    result = reader.read(&mut buffer) => {
                        match result {
                            Ok(_) => {
                                thread_safe_robot.lock().await.handle_packet(buffer.clone(), thread_safe_robot.clone()).await;
                            },
                            Err(_) => {
                                break
                            }
                        }
                    }
                    _ = rx.recv() => {
                        debug!("Closing the tcp socket.");

                        break
                    }
                }
            }
            {
                debug!("Destroyed tcp socket.");
                let mut locked_robot = thread_safe_robot.lock().await;
                let mut_robot = locked_robot.borrow_mut();
                if mut_robot.state.is_some() {
                    mut_robot
                        .robot_map
                        .lock()
                        .await
                        .remove(&mut_robot.state.as_ref().unwrap().team_number);
                    mut_robot.has_closed = true;
                }
            }
        });
    }

    async fn send_station_info(&mut self) {
        let mut buffer: Vec<u8> = vec![0x19, 0x01, 0x00];

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await;
    }

    async fn send_event_name(&mut self) {
        let name = "test";
        let length = name.len() as u8;
        let mut buffer: Vec<u8> = vec![0x14, length];
        buffer.extend_from_slice(name.as_bytes());

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await;
    }

    pub(crate) async fn send_udp_state(&mut self) {
        let mut buffer: Vec<u8> = vec![0; 22];

        let mut state = self.state.as_mut().unwrap();

        buffer[0] = (state.sequence_number >> 8 & 0xff) as u8;
        buffer[1] = (state.sequence_number & 0xff) as u8;

        buffer[2] = 0;
        buffer[3] = 0; // Control Byte
        if state.mode == Mode::Autonomous {
            buffer[3] |= 0x02
        }

        if state.enable && !self.field_override.disabled {
            buffer[3] |= 0x04
        }

        if state.emergency_stop || self.field_override.emergency_stop {
            buffer[3] |= 0x80
        }

        buffer[4] = 0; // Unknown
        buffer[5] = state.alliance_station.to_integer() as u8; // Station Number
        buffer[6] = (state.match_number >> 8 & 0xff) as u8; // Match Number
        buffer[7] = (state.match_number & 0xff) as u8;

        buffer[9] = 1; // Replay number.

        let time = Local::now();
        buffer[10] = (((time.nanosecond() / 1000) >> 24) & 0xff) as u8; // Timestamp
        buffer[11] = (((time.nanosecond() / 1000) >> 16) & 0xff) as u8;
        buffer[12] = (((time.nanosecond() / 1000) >> 8) & 0xff) as u8;
        buffer[13] = ((time.nanosecond() / 1000) & 0xff) as u8;
        buffer[14] = time.second() as u8;
        buffer[15] = time.minute() as u8;
        buffer[16] = time.hour() as u8;
        buffer[17] = time.day() as u8;
        buffer[18] = time.month() as u8;
        buffer[19] = (time.year() - 1900) as u8;

        buffer[20] = (state.time_to_display >> 8 & 0xff) as u8; // Match Time

        buffer[21] = (state.time_to_display & 0xff) as u8;

        let remote_address = SocketAddr::new(self.socket_address.ip(), 1121);
        self.udp_socket.send_to(&buffer, remote_address).await;

        // Reset to zero if the sequence number gets to high.
        if state.sequence_number + 1 > u16::MAX {
            state.sequence_number = 0
        }
        state.sequence_number += 1;
    }

    async fn handle_packet(&mut self, buffer: Vec<u8>, robot: ThreadSafeRobot) {
        match buffer[2] {
            0x18 => {
                let team_number = (((buffer[3] as i32) << 8) + (buffer[4] as i32)) as u16;

                let alliance_station_map = {
                    let locked_robot = robot.lock().await;

                    locked_robot.alliance_station_map.clone()
                };

                let locked_alliance_station_map = alliance_station_map.lock().await;

                let alliance_station = if locked_alliance_station_map.contains_key(&team_number) {
                    *locked_alliance_station_map.get(&team_number).unwrap()
                } else {
                    AllianceStation::None
                };

                let status = if alliance_station != AllianceStation::None {
                    DriverstationStatus::Good
                } else {
                    DriverstationStatus::Good
                };

                self.state = Option::Some(State {
                    emergency_stop: false,
                    enable: false,
                    mode: Mode::TeleOp,
                    team_number,
                    alliance_station,
                    status,
                    sequence_number: 0,
                    time_to_display: 0,
                    match_number: 0,
                    event_name: self.original_event_name.clone()
                });

                let mut locked_robot = robot.lock().await;

                // TODO: It may be necessary to bring in a second Robot Map for all robots and verified robots.
                if status == DriverstationStatus::Good {
                    locked_robot
                        .robot_map
                        .clone()
                        .lock()
                        .await
                        .insert(team_number, robot.clone());
                }

                locked_robot.send_station_info().await;
                locked_robot.send_event_name().await;
            }
            _ => {}
        }
    }

    pub(crate) fn update_confirmed_state(&mut self, state: ConfirmedState) {
        self.confirmed_state = Option::Some(state)
    }
}

fn prefix_with_size(buffer: Vec<u8>) -> Vec<u8> {
    let length = buffer.len();
    let mut new_buffer: Vec<u8> = vec![0; 2];
    new_buffer[0] = (length >> 8 & 0xff) as u8;
    new_buffer[1] = (length & 0xff) as u8;
    new_buffer.extend_from_slice(&buffer);
    return new_buffer;
}
