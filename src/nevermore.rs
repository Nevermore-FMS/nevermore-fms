use crate::robot::{ConfirmedState, Mode, Robot, ThreadSafeRobot};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::broadcast::Sender;
use tokio::sync::Mutex;

use std::str;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::Duration;

pub type RobotMap = Arc<Mutex<HashMap<u16, ThreadSafeRobot>>>;

pub struct Nevermore {
    pub team_number_to_robot: RobotMap,
    udp_socket: Option<Arc<UdpSocket>>,
    pub closing_sender: Option<Sender<()>>,
    pub ticker_sender: Sender<()>,
}

impl Nevermore {
    pub async fn new() -> anyhow::Result<Arc<Mutex<Nevermore>>> {
        let (tx, _) = tokio::sync::broadcast::channel(10);

        let nevermore = Arc::new(Mutex::new(Nevermore {
            team_number_to_robot: Arc::new(Mutex::new(HashMap::new())),
            udp_socket: Option::None,
            closing_sender: Option::None,
            ticker_sender: tx,
        }));

        Ok(nevermore)
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let (tx, mut rx1) = tokio::sync::broadcast::channel(1);
        let mut rx2 = tx.subscribe();
        self.closing_sender = Option::Some(tx);

        self.listen_for_udp_messages("10.0.100.5:1160".parse()?, rx2)
            .await?;
        self.listen_for_tcp_connections("10.0.100.5:1750".parse()?, rx1)
            .await?;
        self.start_ticking().await;

        /*let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
        while let Some(line) = input_lines.next_line().await? {
            println!("length = {}", line.len())
        }*/

        Ok(())
    }

    async fn start_ticking(&self) {
        let robot_map = self.team_number_to_robot.clone();
        let ticker_sender = self.ticker_sender.clone();

        tokio::spawn(async move {
            loop {
                for (_, robot) in robot_map.lock().await.iter() {
                    info!("Got robot!");
                    robot.lock().await.send_udp_state().await;
                }
                ticker_sender.send(());
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }

    async fn listen_for_tcp_connections(
        &self,
        tcp_address: SocketAddr,
        mut closing_channel: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let listener = TcpListener::bind(tcp_address).await?;
        let robot_map = self.team_number_to_robot.clone();
        let udp_socket = self.udp_socket.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    socket = listener.accept() => {
                        match socket {
                            Ok((stream, address)) => {
                                let cloned_udp_socket = udp_socket.clone();
                                if cloned_udp_socket.is_some() {
                                    Robot::handle_connection(stream, address, robot_map.clone(), cloned_udp_socket.unwrap()).await;
                                }
                            },
                            Err(_) => {
                                return
                            }
                        }
                    }
                    _ = closing_channel.recv() => {
                        info!("Closing the tcp listener.");

                        return
                    }
                }
            }
        });

        Ok(())
    }

    async fn listen_for_udp_messages(
        &mut self,
        udp_address: SocketAddr,
        mut closing_channel: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let listener = Arc::new(UdpSocket::bind(udp_address).await?);
        let robot_map = self.team_number_to_robot.clone();

        self.udp_socket = Option::Some(listener.clone());

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                tokio::select! {
                    result = listener.recv_from(&mut buf) => {
                        match result {
                            Ok((_, address)) => {
                                Self::decode_udp_message(robot_map.clone(), address, buf.clone()).await;
                            },
                            Err(_) => {
                                return
                            }
                        }
                    }
                    _ = closing_channel.recv() => {
                        info!("Closing the udp listener.");

                        return
                    }
                }
            }
        });

        Ok(())
    }

    async fn decode_udp_message(
        robot_map: RobotMap,
        from: SocketAddr,
        buffer: Vec<u8>,
    ) -> anyhow::Result<()> {
        let is_emergency_stopped = ((buffer[3] as i32) >> 7 & 0x01) == 1;
        let robot_comms_active = ((buffer[3] as i32) >> 5 & 0x01) == 1;
        let can_ping_radio = ((buffer[3] as i32) >> 4 & 0x01) == 1;
        let can_ping_rio = ((buffer[3] as i32) >> 3 & 0x01) == 1;
        let is_enabled = ((buffer[3] as i32) >> 2 & 0x01) == 1;

        let mode = Mode::from_integer((buffer[3] as i32) & 0x03);

        let team_number = (((buffer[4] as i32) << 8) + (buffer[5] as i32)) as u16;
        let battery_voltage = (buffer[6] as f32) + ((buffer[7] as f32) / 256.0);

        info!("{}", team_number);

        robot_map
            .lock()
            .await
            .get_mut(&team_number)
            .ok_or(anyhow::anyhow!("team number doesn't exist"))?
            .lock()
            .await
            .update_confirmed_state(ConfirmedState {
                is_emergency_stopped,
                robot_comms_active,
                can_ping_radio,
                can_ping_rio,
                is_enabled,
                mode,
                team_number,
                battery_voltage,
            });

        Ok(())
    }
}
