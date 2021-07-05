use crate::field::driverstation::{ConfirmedState, DriverStation, State, ThreadSafeDriverStation};
use crate::field::enums::{AllianceStation, Mode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use tokio::time::Duration;
use log::info;

pub mod driverstation;
pub mod enums;

pub type ThreadSafeField = Arc<Mutex<Field>>;

pub type ThreadSafeDriverStationMap = Arc<Mutex<HashMap<u16, ThreadSafeDriverStation>>>;

pub type ThreadSafeAllianceStationMap = Arc<Mutex<HashMap<u16, AllianceStation>>>;

pub type ThreadSafeStateMap = Arc<Mutex<HashMap<u16, State>>>;

pub type ThreadSafeFieldOverride = Arc<RwLock<FieldOverride>>;

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldOverride {
    pub emergency_stop: bool,
    pub disabled: bool,
}

pub struct Field {
    team_number_to_robot: ThreadSafeDriverStationMap,
    team_number_to_state: ThreadSafeStateMap,
    team_number_to_station: ThreadSafeAllianceStationMap,
    udp_socket: Arc<UdpSocket>,
    closing_sender: Sender<()>,
    ticker_sender: Sender<()>,
    event_name: String,
    field_override: ThreadSafeFieldOverride,
}

impl Field {
    /// Emergency stops all robots on the field globally, cannot be overriden.
    pub async fn set_emergency_stop_all(&self, emergency_stop: bool) -> anyhow::Result<()> {
        self.field_override.write().await.emergency_stop = emergency_stop;
        Ok(())
    }

    /// Disables all robots on the field globally, cannot be overriden.
    pub async fn set_enabled_all(&self, enabled: bool) -> anyhow::Result<()> {
        self.field_override.write().await.disabled = !enabled;
        Ok(())
    }

    /// Adds a team to the `ThreadSafeAllianceStationMap`, this allows the team to properly connect to the FMS.
    pub async fn add_team(&self, team_number: u16, station: AllianceStation) -> anyhow::Result<()> {
        self.team_number_to_station
            .lock()
            .await
            .insert(team_number, station);
        Ok(())
    }

    /// Removes a team from the `ThreadSafeAllianceStationMap`
    pub async fn remove_team(&self, team_number: u16) -> anyhow::Result<()> {
        self.team_number_to_station
            .lock()
            .await
            .remove(&team_number)
            .ok_or(anyhow::anyhow!("team doesn't exist"))?;
        Ok(())
    }

    /// Retrieves a team from the `ThreadSafeAllianceStationMap`
    pub async fn get_team_alliance_station(
        &self,
        team_number: u16,
    ) -> anyhow::Result<AllianceStation> {
        Ok(self
            .team_number_to_station
            .lock()
            .await
            .get(&team_number)
            .ok_or(anyhow::anyhow!("team doesn't exist"))?
            .clone())
    }

    // Retrieves the Team to Alliance Station map.
    pub async fn get_team_alliance_station_map(
        &self,
    ) -> anyhow::Result<HashMap<u16, AllianceStation>> {
        Ok(self.team_number_to_station.lock().await.clone())
    }

    /// Retrieves a driver station by it's team number, keep in mind that once the driver station disconnects the
    /// `ThreadSafeDriverStation` returned here is useless. TODO: Add a function to the driver station to check if
    /// it still exists in the driver station.
    pub async fn get_driver_station(
        &self,
        team_number: u16,
    ) -> anyhow::Result<ThreadSafeDriverStation> {
        Ok(self
            .team_number_to_robot
            .lock()
            .await
            .get(&team_number)
            .ok_or(anyhow::anyhow!("team number not in map"))?
            .clone())
    }

    /// Retrieves all connected driver stations, keep in mind that once the driver station disconnects the
    /// `ThreadSafeDriverStation` returned here is useless. TODO: Add a function to the driver station to check if
    /// it still exists in the driver station.
    pub async fn driver_stations(&self) -> anyhow::Result<Vec<ThreadSafeDriverStation>> {
        Ok(self
            .team_number_to_robot
            .lock()
            .await
            .values()
            .cloned()
            .collect())
    }

    /// Retrieves the team numbers of all connected driver stations, keep in mind that once the driver station disconnects the
    /// `ThreadSafeDriverStation` returned here is useless. TODO: Add a function to the driver station to check if
    /// it still exists in the driver station.
    pub async fn driver_station_team_numbers(&self) -> anyhow::Result<Vec<u16>> {
        Ok(self
            .team_number_to_robot
            .lock()
            .await
            .keys()
            .map(|x| {
                *x
            })
            .collect())
    }

    /// Retrieves a `Reciever<()>` that is called when the `Field` is being shut down.
    pub fn subscribe_to_close_channel(&self) -> anyhow::Result<Receiver<()>> {
        Ok(self.closing_sender.subscribe())
    }

    /// Retrieves a `Reciever<()>` that is called when the Field finishes a tick.
    /// TODO: Consider adding a pre and post tick channel.
    pub fn subscribe_to_tick_channel(&self) -> anyhow::Result<Receiver<()>> {
        Ok(self.ticker_sender.subscribe())
    }

    // Gets the event name sent to robots.
    pub fn get_event_name(&self) -> String {
        self.event_name.clone()
    }

    // Sets the event name sent to robots.
    pub fn set_event_name(&mut self, event_name: String) {
        self.event_name = event_name;
    }

    // Internal API -->

    pub(crate) async fn new(event_name: String) -> anyhow::Result<ThreadSafeField> {
        let (ticker_sender, _) = tokio::sync::broadcast::channel(10);

        let (closing_sender, rx1) = tokio::sync::broadcast::channel(1);
        let rx2 = closing_sender.subscribe();

        let udp_address: SocketAddr = "10.0.100.5:1160".parse()?;

        let udp_socket = Arc::new(UdpSocket::bind(udp_address).await?);

        let field = Arc::new(Mutex::new(Field {
            team_number_to_robot: Arc::new(Mutex::new(HashMap::new())),
            team_number_to_state: Arc::new(Mutex::new(HashMap::new())),
            team_number_to_station: Arc::new(Mutex::new(HashMap::new())),
            udp_socket: udp_socket.clone(),
            closing_sender,
            ticker_sender,
            event_name,
            field_override: Arc::new(RwLock::new(FieldOverride {
                emergency_stop: false,
                disabled: false,
            })),
        }));

        Self::listen_for_udp_messages(field.clone(), udp_socket.clone(), rx2).await?;
        Self::listen_for_tcp_connections(field.clone(), "10.0.100.5:1750".parse()?, rx1).await?;
        Self::start_ticking(field.clone()).await;

        Ok(field.clone())
    }

    async fn start_ticking(field: ThreadSafeField) {
        let locked_field = field.lock().await;
        let driver_station_map = locked_field.team_number_to_robot.clone();
        let ticker_sender = locked_field.ticker_sender.clone();

        tokio::spawn(async move {
            loop {
                for (_, robot) in driver_station_map.lock().await.iter() {
                    robot.lock().await.send_udp_state().await;
                }
                ticker_sender.send(()).ok();
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });
    }

    async fn listen_for_tcp_connections(
        field: ThreadSafeField,
        tcp_address: SocketAddr,
        mut closing_channel: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let listener = TcpListener::bind(tcp_address).await?;
        let cloned_field = field.clone();
        let locked_field = cloned_field.lock().await;
        let driver_station_map = locked_field.team_number_to_robot.clone();
        let alliance_station_map = locked_field.team_number_to_station.clone();
        let state_map = locked_field.team_number_to_state.clone();
        let field_override = locked_field.field_override.clone();
        let udp_socket = locked_field.udp_socket.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    socket = listener.accept() => {
                        match socket {
                            Ok((stream, address)) => {
                                let cloned_field = field.clone();
                                let event_name = &cloned_field.lock().await.event_name;

                                DriverStation::handle_connection(stream, address, driver_station_map.clone(), alliance_station_map.clone(), state_map.clone(), field_override.clone(), udp_socket.clone(), event_name.clone()).await;
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
        field: ThreadSafeField,
        udp_socket: Arc<UdpSocket>,
        mut closing_channel: tokio::sync::broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let locked_field = field.lock().await;
        let driver_station_map = locked_field.team_number_to_robot.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                tokio::select! {
                    result = udp_socket.recv_from(&mut buf) => {
                        match result {
                            Ok((_, address)) => {
                                Self::decode_udp_message(driver_station_map.clone(), address, buf.clone()).await.ok();
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
        driver_station_map: ThreadSafeDriverStationMap,
        _from: SocketAddr,
        buffer: Vec<u8>,
    ) -> anyhow::Result<()> {
        let is_emergency_stopped = ((buffer[3] as i32) >> 7 & 0x01) == 1;
        let robot_communications_active = ((buffer[3] as i32) >> 5 & 0x01) == 1;
        let can_ping_radio = ((buffer[3] as i32) >> 4 & 0x01) == 1;
        let can_ping_rio = ((buffer[3] as i32) >> 3 & 0x01) == 1;
        let is_enabled = ((buffer[3] as i32) >> 2 & 0x01) == 1;

        let mode = Mode::from_integer((buffer[3] as i32) & 0x03);

        let team_number = (((buffer[4] as i32) << 8) + (buffer[5] as i32)) as u16;
        let battery_voltage = (buffer[6] as f32) + ((buffer[7] as f32) / 256.0);

        driver_station_map
            .lock()
            .await
            .get_mut(&team_number)
            .ok_or(anyhow::anyhow!("team number doesn't exist"))?
            .lock()
            .await
            .update_confirmed_state(ConfirmedState {
                is_emergency_stopped,
                robot_communications_active,
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

impl Drop for Field {
    fn drop(&mut self) {
        self.closing_sender.send(()).ok();
    }
}
