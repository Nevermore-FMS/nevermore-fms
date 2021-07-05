use chrono::{Datelike, Local, Timelike};

use serde::{Deserialize, Serialize};

use crate::field::enums::{AllianceStation, DriverstationStatus, Mode};
use crate::field::{
    ThreadSafeAllianceStationMap, ThreadSafeDriverStationMap, ThreadSafeFieldOverride,
    ThreadSafeStateMap,
};
use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;
use log::debug;

pub type ThreadSafeDriverStation = Arc<Mutex<DriverStation>>;

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

#[derive(Clone, Serialize, Deserialize)]
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

pub struct DriverStation {
    confirmed_state: Option<ConfirmedState>,
    team_number: u16,
    state_map: ThreadSafeStateMap,
    field_override: ThreadSafeFieldOverride,
    tcp_writer: OwnedWriteHalf,
    udp_socket: Arc<UdpSocket>,
    socket_address: SocketAddr,
    driver_station_map: ThreadSafeDriverStationMap,
    alliance_station_map: ThreadSafeAllianceStationMap,
    closing_sender: Sender<()>,
    original_event_name: String,
    has_closed: bool,
}

impl DriverStation {
    pub fn get_confirmed_state(&self) -> anyhow::Result<ConfirmedState> {
        self.confirmed_state
            .ok_or(anyhow::anyhow!("confirmed_state hasn't been formed yet"))
    }

    pub async fn get_state(&self) -> anyhow::Result<State> {
        Ok(self
            .state_map
            .lock()
            .await
            .get(&self.team_number)
            .ok_or(anyhow::anyhow!("state not formed yet"))?
            .clone())
    }

    pub async fn set_state(&mut self, state: State) {
        let state_map = self.state_map.clone();
        let mut locked_state_map = state_map.lock().await;

        let old_state_maybe = locked_state_map.insert(self.team_number, state.clone());

        if old_state_maybe.is_some() {
            let old_state = old_state_maybe.clone().unwrap();
            if old_state.event_name != state.event_name
                || old_state.alliance_station != state.alliance_station
                || old_state.status != state.status
            {
                drop(locked_state_map); // Unlock mutex on state map.
                self.send_event_name().await;
                self.send_station_info().await;
            }
        }
    }

    pub async fn is_in_correct_station(&self) -> anyhow::Result<bool> {
        Ok(self.get_state().await?.status == DriverstationStatus::Good)
    }

    pub async fn is_in_match(&self) -> anyhow::Result<bool> {
        Ok(self.get_state().await?.status != DriverstationStatus::Waiting)
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
        driver_station_map: ThreadSafeDriverStationMap,
        alliance_station_map: ThreadSafeAllianceStationMap,
        state_map: ThreadSafeStateMap,
        field_override: ThreadSafeFieldOverride,
        udp_socket: Arc<UdpSocket>,
        event_name: String,
    ) {
        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::broadcast::channel::<()>(1);
            let (mut reader, writer) = socket.into_split();
            let robot = DriverStation {
                confirmed_state: None,
                field_override,
                tcp_writer: writer,
                udp_socket,
                socket_address,
                driver_station_map,
                alliance_station_map,
                closing_sender: tx,
                original_event_name: event_name,
                has_closed: false,
                team_number: 0,
                state_map,
            };

            let thread_safe_robot = Arc::new(Mutex::new(robot));

            let mut buffer: Vec<u8> = vec![0; 50];

            loop {
                tokio::select! {
                    result = reader.read(&mut buffer) => {
                        match result {
                            Ok(_) => {
                                Self::handle_packet(buffer.clone(), thread_safe_robot.clone()).await;
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
                let mut locked_robot = thread_safe_robot.lock().await;
                let mut_robot = locked_robot.borrow_mut();
                debug!("Destroyed tcp socket for {}.", mut_robot.team_number);
                mut_robot
                    .driver_station_map
                    .lock()
                    .await
                    .remove(&mut_robot.team_number);
                mut_robot.has_closed = true;
            }
        });
    }

    async fn send_station_info(&mut self) {
        let state = self.get_state().await.unwrap();
        let mut buffer: Vec<u8> = vec![
            0x19,
            state.alliance_station.to_integer() as u8,
            state.status.to_integer() as u8,
        ];

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await.ok();
    }

    async fn send_event_name(&mut self) {
        let name = self.get_state().await.unwrap().event_name;
        let length = name.len() as u8;
        let mut buffer: Vec<u8> = vec![0x14, length];
        buffer.extend_from_slice(name.as_bytes());

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await.ok();
    }

    pub(crate) async fn send_udp_state(&mut self) {
        let mut buffer: Vec<u8> = vec![0; 22];

        let mut state = self.get_state().await.unwrap();

        buffer[0] = (state.sequence_number >> 8 & 0xff) as u8;
        buffer[1] = (state.sequence_number & 0xff) as u8;

        buffer[2] = 0;
        buffer[3] = 0; // Control Byte
        if state.mode == Mode::Autonomous {
            buffer[3] |= 0x02
        }

        let field_override = self.field_override.read().await;

        if state.enable && !field_override.disabled {
            buffer[3] |= 0x04
        }

        if state.emergency_stop || field_override.emergency_stop {
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
        self.udp_socket.send_to(&buffer, remote_address).await.ok();

        // Reset to zero if the sequence number gets to high.
        if state.sequence_number + 1 > u16::MAX {
            state.sequence_number = 0
        }
        state.sequence_number += 1;
    }

    async fn handle_packet(buffer: Vec<u8>, robot: ThreadSafeDriverStation) {
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
                    DriverstationStatus::Waiting
                };

                let mut locked_robot = robot.lock().await;

                locked_robot.team_number = team_number;

                let event_name = locked_robot.original_event_name.clone();

                locked_robot
                    .set_state(State {
                        emergency_stop: false,
                        enable: false,
                        mode: Mode::TeleOp,
                        team_number,
                        alliance_station,
                        status,
                        sequence_number: 0,
                        time_to_display: 0,
                        match_number: 0,
                        event_name,
                    })
                    .await;

                locked_robot
                    .driver_station_map
                    .clone()
                    .lock()
                    .await
                    .insert(team_number, robot.clone());

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

impl Drop for DriverStation {
    fn drop(&mut self) {
        self.closing_sender.send(()).ok();
    }
}
