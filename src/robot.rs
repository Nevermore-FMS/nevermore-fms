use crate::nevermore::RobotMap;
use chrono::{Datelike, Local, Timelike};
use deno_core::Resource;
use serde::Serialize;
use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpSocket, TcpStream, UdpSocket};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

pub type ThreadSafeRobot = Arc<Mutex<Robot>>;

#[derive(Copy, Clone, Serialize)]
pub enum Mode {
    TeleOp,
    Test,
    Autonomous,
}

impl Mode {
    pub fn from_integer(integer: i32) -> Mode {
        match integer {
            0 => Mode::TeleOp,
            1 => Mode::Test,
            2 => Mode::Autonomous,
            _ => Mode::TeleOp,
        }
    }

    pub fn to_integer(self) -> i32 {
        match self {
            Mode::TeleOp => 0,
            Mode::Test => 1,
            Mode::Autonomous => 2,
        }
    }
}

#[derive(Copy, Clone, Serialize)]
pub struct ConfirmedState {
    pub is_emergency_stopped: bool,
    pub robot_comms_active: bool,
    pub can_ping_radio: bool,
    pub can_ping_rio: bool,
    pub is_enabled: bool,

    pub mode: Mode,

    pub team_number: u16,
    pub battery_voltage: f32,
}

#[derive(Copy, Clone)]
pub struct State {
    pub emergency_stopped: bool,
    pub enable: bool,

    pub mode: Mode,

    pub team_number: u16,
    pub sequence_number: u16,
    pub time_to_display: u16,
    pub match_number: u16,
}

pub struct Robot {
    pub confirmed_state: Option<ConfirmedState>,
    pub state: Option<State>,
    pub tcp_writer: OwnedWriteHalf,
    pub udp_socket: Arc<UdpSocket>,
    pub socket_address: SocketAddr,
    pub robot_map: RobotMap,
    closing_sender: Sender<()>,
}

impl Robot {
    // Remember everything is big endian with the FMS because they like suffering.
    pub async fn handle_connection(
        mut socket: TcpStream,
        socket_address: SocketAddr,
        robot_map: RobotMap,
        udp_socket: Arc<UdpSocket>,
    ) {
        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
            let (mut reader, writer) = socket.into_split();
            let mut robot = Robot {
                confirmed_state: None,
                state: None,
                tcp_writer: writer,
                udp_socket,
                socket_address,
                robot_map,
                closing_sender: tx,
            };

            let thread_safe_robot = Arc::new(Mutex::new(robot));

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
                let locked_robot = thread_safe_robot.lock().await;
                if locked_robot.state.is_some() {
                    locked_robot
                        .robot_map
                        .lock()
                        .await
                        .remove(&locked_robot.state.unwrap().team_number);
                }
            }
        });
    }

    pub async fn send_station_info(&mut self) {
        let mut buffer: Vec<u8> = vec![0x19, 0x01, 0x00];

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await;
    }

    pub async fn send_event_name(&mut self) {
        let name = "test";
        let length = name.len() as u8;
        let mut buffer: Vec<u8> = vec![0x14, length];
        buffer.extend_from_slice(name.as_bytes());

        buffer = prefix_with_size(buffer);
        self.tcp_writer.write(&buffer).await;
    }

    pub async fn send_udp_state(&mut self) {
        let mut buffer: Vec<u8> = vec![0; 22];

        let mut state = self.state.as_mut().unwrap();

        buffer[0] = (state.sequence_number >> 8 & 0xff) as u8;
        buffer[1] = (state.sequence_number & 0xff) as u8;

        buffer[2] = 0;
        buffer[3] = 0; // Control Byte

        buffer[4] = 0;
        buffer[5] = 0; // station number
        buffer[6] = (state.match_number >> 8 & 0xff) as u8;
        buffer[7] = (state.match_number & 0xff) as u8;

        buffer[9] = 1; // Replay number.

        let time = Local::now();
        buffer[10] = (((time.nanosecond() / 1000) >> 24) & 0xff) as u8;
        buffer[11] = (((time.nanosecond() / 1000) >> 16) & 0xff) as u8;
        buffer[12] = (((time.nanosecond() / 1000) >> 8) & 0xff) as u8;
        buffer[13] = ((time.nanosecond() / 1000) & 0xff) as u8;
        buffer[14] = time.second() as u8;
        buffer[15] = time.minute() as u8;
        buffer[16] = time.hour() as u8;
        buffer[17] = time.day() as u8;
        buffer[18] = time.month() as u8;
        buffer[19] = (time.year() - 1900) as u8;

        buffer[20] = (state.time_to_display >> 8 & 0xff) as u8;

        buffer[21] = (state.time_to_display & 0xff) as u8;

        let remote_address = SocketAddr::new(self.socket_address.ip(), 1121);
        self.udp_socket.send_to(&buffer, remote_address).await;

        state.sequence_number += 1;
    }

    pub async fn handle_packet(&mut self, buffer: Vec<u8>, robot: ThreadSafeRobot) {
        //let length = buffer[0];
        debug!("Got packet {}", buffer[2]);
        match buffer[2] {
            0x18 => {
                let team_number = (((buffer[3] as i32) << 8) + (buffer[4] as i32)) as u16;

                debug!("Got packet from {}", team_number);

                self.state = Option::Some(State {
                    emergency_stopped: false,
                    enable: false,
                    mode: Mode::TeleOp,
                    team_number,
                    sequence_number: 0,
                    time_to_display: 0,
                    match_number: 0,
                });

                self.robot_map
                    .clone()
                    .lock()
                    .await
                    .insert(team_number, robot);

                self.send_station_info().await;
                self.send_event_name().await;
            }
            _ => {}
        }
    }

    pub fn update_confirmed_state(&mut self, state: ConfirmedState) {
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
