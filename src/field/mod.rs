pub mod connection;
pub mod driverstation;
pub mod enums;

use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use log::*;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{
        broadcast::{self},
        RwLock,
    },
    time,
};

use crate::{alarms::FMSAlarmHandler, difftimer};

use self::{driverstation::DriverStations, enums::TournamentLevel};

struct RawField {
    event_name: String,
    tournament_level: TournamentLevel,
    match_number: u16,
    play_number: u8,
    time_left: difftimer::DiffTimer,
    ds_mode: enums::Mode,
    driverstations: DriverStations,
    alarm_handler: FMSAlarmHandler,
    is_safe: bool,
    terminate_signal: Option<broadcast::Sender<()>>,
    running_signal: async_channel::Receiver<()>,
    udp_online: bool,
    tcp_online: bool,
}

#[derive(Clone)]
pub struct Field {
    raw: Arc<RwLock<RawField>>,
}

impl Field {
    // Public API -->
    pub async fn terminate(&self) {
        let mut raw_field = self.raw.write().await;
        drop(raw_field.terminate_signal.take());
    }

    pub async fn wait_for_terminate(&self) {
        let raw = self.raw.read().await;
        let running_signal = raw.running_signal.clone();
        drop(raw);
        let _ = running_signal.recv().await;
    }

    pub async fn udp_online(&self) -> bool {
        let raw = self.raw.read().await;
        raw.udp_online
    }

    pub async fn tcp_online(&self) -> bool {
        let raw = self.raw.read().await;
        raw.tcp_online
    }

    pub async fn driverstations(&self) -> DriverStations {
        let raw = self.raw.read().await;
        raw.driverstations.clone()
    }

    pub async fn alarm_handler(&self) -> FMSAlarmHandler {
        let raw = self.raw.read().await;
        raw.alarm_handler.clone()
    }

    pub async fn alarm_target(&self) -> String {
        return format!("fms.field");
    }

    pub async fn event_name(&self) -> String {
        let raw = self.raw.read().await;
        raw.event_name.clone()
    }

    pub async fn set_event_name(&self, event_name: String) {
        let mut raw = self.raw.write().await;
        raw.event_name = event_name;
        info!("Event name set to {}", raw.event_name.clone());
    }

    pub async fn tournament_level(&self) -> TournamentLevel {
        let raw = self.raw.read().await;
        raw.tournament_level.clone()
    }

    pub async fn set_tournament_level(&self, tournament_level: TournamentLevel) {
        let mut raw = self.raw.write().await;
        raw.tournament_level = tournament_level;
        info!("Tournament Level set to {}", raw.tournament_level.clone());
    }

    pub async fn match_number(&self) -> u16 {
        let raw = self.raw.read().await;
        raw.match_number.clone()
    }

    pub async fn set_match_number(&self, match_number: u16) {
        let mut raw = self.raw.write().await;
        raw.match_number = match_number;
        info!("Match Number set to {}", &raw.match_number);
    }

    pub async fn play_number(&self) -> u8 {
        let raw = self.raw.read().await;
        raw.play_number.clone()
    }

    pub async fn set_play_number(&self, play_number: u8) {
        let mut raw = self.raw.write().await;
        raw.play_number = play_number;
        info!("Play number set to {}", &raw.play_number);
    }

    pub async fn timer(&self) -> difftimer::DiffTimer {
        let raw = self.raw.read().await;
        raw.time_left.clone()
    }

    pub async fn set_time_remaining(&self, time_left: Duration) {
        let mut raw = self.raw.write().await;
        raw.time_left = difftimer::DiffTimer::new(time_left, raw.time_left.is_running());
        info!("Timer set to {} ms", time_left.as_millis());
    }

    pub async fn start_timer(&self) {
        let mut raw = self.raw.write().await;
        if !raw.time_left.is_running() {
            raw.time_left = raw.time_left.start();
            info!("Timer started");
        }
    }

    pub async fn stop_timer(&self) {
        let mut raw = self.raw.write().await;
        if raw.time_left.is_running() {
            raw.time_left = raw.time_left.stop();
            info!("Timer stopped");
        }
    }

    pub async fn match_abort(&self) {
        self.stop_timer().await
        //TODO Other actions related to match abort
    }

    pub async fn ds_mode(&self) -> enums::Mode {
        let raw: tokio::sync::RwLockReadGuard<RawField> = self.raw.read().await;
        raw.ds_mode
    }

    pub async fn set_ds_mode(&self, ds_mode: enums::Mode) {
        let mut raw = self.raw.write().await;
        raw.ds_mode = ds_mode;
        info!("DS Mode set to {}", ds_mode);
    }

    pub async fn is_safe(&self) -> bool {
        let raw = self.raw.read().await;
        raw.is_safe
    }

    pub async fn set_is_safe(&self, is_safe: bool) {
        let mut raw = self.raw.write().await;
        raw.is_safe = is_safe;
        info!("Field safe flag set to {}", is_safe);
    }

    // Internal API -->

    pub(super) async fn new(ds_address: IpAddr) -> anyhow::Result<Self> {
        let (terminate_sender, _) = broadcast::channel(1);

        let (indicate_running, running_signal) = async_channel::bounded(1);

        let field = RawField {
            event_name: "nvmre".to_string(),
            tournament_level: TournamentLevel::Test,
            match_number: 1,
            play_number: 1,
            time_left: difftimer::DiffTimer::new(Duration::ZERO, false),
            ds_mode: enums::Mode::Autonomous,
            driverstations: DriverStations::new(None),
            alarm_handler: FMSAlarmHandler::new(),
            is_safe: true,
            terminate_signal: Some(terminate_sender),
            running_signal,
            udp_online: false,
            tcp_online: false,
        };

        let field = Self {
            raw: Arc::new(RwLock::new(field)),
        };

        field
            .driverstations()
            .await
            .set_field(field.clone())
            .await?;

        let udp_address = SocketAddr::new(ds_address, 1160);
        let tcp_address = SocketAddr::new(ds_address, 1750);
        let async_field = field.clone();
        tokio::spawn(async move {
            let (udp_result, tcp_result, _) = tokio::join!(
                async_field.listen_for_udp_messages(udp_address),
                async_field.listen_for_tcp_connections(tcp_address),
                async_field.tick_loop()
            );
            udp_result.unwrap();
            tcp_result.unwrap();
            drop(indicate_running);
        });

        Ok(field)
    }

    async fn listen_for_udp_messages(&self, addr: SocketAddr) -> anyhow::Result<()> {
        loop {
            //Retry Loop
            let mut raw_field = self.raw.write().await;
            let socket = UdpSocket::bind(addr).await.context(bind_err("UDP", addr));
            if socket.is_err() {
                drop(raw_field);
                error!("{}", socket.err().unwrap());
                tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                continue;
            }
            let socket = socket.unwrap();
            raw_field.udp_online = true;

            let mut term_rx = raw_field
                .terminate_signal
                .as_ref()
                .context("Can't listen for UDP Messages because field has already terminated")
                .unwrap()
                .subscribe();
            let driverstations = raw_field.driverstations.clone();
            drop(raw_field);

            let mut buf = vec![0; 1024];
            info!("Listening for UDP messages on {}", addr);
            loop {
                tokio::select! {
                    result = socket.recv_from(&mut buf) => {
                        match result {
                            Ok((size, _)) => {
                                if let Err(e) = driverstations.decode_udp_message(buf[..size].to_vec()).await {
                                    if e.to_string() != "unexpected end of file" {
                                        error!("Error decoding UDP message: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Error when reading UDP Message: {}", e);
                            }
                        }
                    }
                    _ = term_rx.recv() => {
                        info!("Closing the UDP listener because the field has terminated.");
                        return Ok(());
                    }
                }
            }
        }
    }

    async fn listen_for_tcp_connections(&self, addr: SocketAddr) -> anyhow::Result<()> {
        loop {
            //Retry Loop
            let mut raw_field = self.raw.write().await;
            let listener = TcpListener::bind(addr).await.context(bind_err("TCP", addr));
            if listener.is_err() {
                error!("{}", listener.err().unwrap());
                drop(raw_field);
                tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                continue;
            }
            let listener = listener.unwrap();
            raw_field.tcp_online = true;
            let mut term_rx = raw_field
                .terminate_signal
                .as_ref()
                .context("Can't listen for TCP Connections because field has already terminated")
                .unwrap()
                .subscribe();
            let driverstations = raw_field.driverstations.clone();
            drop(raw_field);

            info!("Listening for TCP connections on {}", addr);
            loop {
                tokio::select! {
                    socket = listener.accept() => {
                        match socket {
                            Ok((stream, socket)) => {
                                if let Err(e) = driverstations.handle_tcp_stream(stream, socket.ip()).await {
                                    error!("Error accepting TCP stream: {}", e);
                                }
                            },
                            Err(e) => {
                                error!("Error when accepting TCP Connection: {}", e);
                            }
                        }
                    }
                    _ = term_rx.recv() => {
                        info!("Closing the TCP listener because the field has terminated.");
                        return Ok(());
                    }
                }
            }
        }
    }

    async fn tick_loop(&self) {
        let raw_field = self.raw.write().await;
        let mut term_rx = raw_field
            .terminate_signal
            .as_ref()
            .context("Can't start the field tick loop because field has already terminated")
            .unwrap()
            .subscribe();
        drop(raw_field);

        let mut interval = time::interval(Duration::from_millis(250));
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
        // Respond to active faults
        if self
            .alarm_handler()
            .await
            .is_target_faulted(self.alarm_target().await.as_str())
            .await
        {
            self.match_abort().await
        }
    }
}

fn bind_err(conn_type: &str, addr: SocketAddr) -> String {
    format!("Coult not bind to {} {}. The host device may not have an interface with that address. To change the ds address, use the --ds-address option. Attempting bind again in 15 seconds.", conn_type, addr)
}
