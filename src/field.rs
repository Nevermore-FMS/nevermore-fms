pub mod connection;
pub mod driverstation;
pub mod enums;

use std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, RwLock},
    time::Duration,
};

use anyhow::Context;
use log::*;
use tokio::{
    net::{TcpListener, UdpSocket},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;

use crate::{alarms::FMSAlarmHandler, difftimer};

use self::{driverstation::DriverStations, enums::TournamentLevel};

struct RawField {
    event_name: String,
    tournament_level: TournamentLevel,
    match_number: u16,
    play_number: u8,
    time_left: difftimer::DiffTimer,
    ds_mode: enums::Mode,
    is_safe: bool,
    udp_online: bool,
    tcp_online: bool,
    driverstations: DriverStations,
    alarm_handler: FMSAlarmHandler,
}

#[derive(Clone)]
pub struct Field {
    raw: Arc<RwLock<RawField>>,
}

impl Field {
    // Public API -->
    pub fn udp_online(&self) -> bool {
        let raw: std::sync::RwLockReadGuard<'_, RawField> = self.raw.read().unwrap();
        raw.udp_online
    }

    pub fn tcp_online(&self) -> bool {
        let raw = self.raw.read().unwrap();
        raw.tcp_online
    }

    pub fn driverstations(&self) -> DriverStations {
        let raw = self.raw.read().unwrap();
        raw.driverstations.clone()
    }

    pub fn alarm_handler(&self) -> FMSAlarmHandler {
        let raw = self.raw.read().unwrap();
        raw.alarm_handler.clone()
    }

    pub fn alarm_target(&self) -> String {
        "fms.field".to_string()
    }

    pub fn event_name(&self) -> String {
        let raw = self.raw.read().unwrap();
        raw.event_name.clone()
    }

    pub fn set_event_name(&self, event_name: String) {
        let mut raw = self.raw.write().unwrap();
        raw.event_name = event_name;
        info!("Event name set to {}", raw.event_name.clone());
    }

    pub fn tournament_level(&self) -> TournamentLevel {
        let raw = self.raw.read().unwrap();
        raw.tournament_level
    }

    pub fn set_tournament_level(&self, tournament_level: TournamentLevel) {
        let mut raw = self.raw.write().unwrap();
        raw.tournament_level = tournament_level;
        info!("Tournament Level set to {}", raw.tournament_level.clone());
    }

    pub fn match_number(&self) -> u16 {
        let raw = self.raw.read().unwrap();
        raw.match_number
    }

    pub fn set_match_number(&self, match_number: u16) {
        let mut raw = self.raw.write().unwrap();
        raw.match_number = match_number;
        info!("Match Number set to {}", &raw.match_number);
    }

    pub fn play_number(&self) -> u8 {
        let raw = self.raw.read().unwrap();
        raw.play_number
    }

    pub fn set_play_number(&self, play_number: u8) {
        let mut raw = self.raw.write().unwrap();
        raw.play_number = play_number;
        info!("Play number set to {}", &raw.play_number);
    }

    pub fn timer(&self) -> difftimer::DiffTimer {
        let raw = self.raw.read().unwrap();
        raw.time_left.clone()
    }

    pub fn set_time_remaining(&self, time_left: Duration) {
        let mut raw = self.raw.write().unwrap();
        raw.time_left = difftimer::DiffTimer::new(time_left, raw.time_left.is_running());
        info!("Timer set to {} ms", time_left.as_millis());
    }

    pub fn start_timer(&self) {
        let mut raw = self.raw.write().unwrap();
        if !raw.time_left.is_running() {
            raw.time_left = raw.time_left.start();
            info!("Timer started");
        }
    }

    pub fn stop_timer(&self) {
        let mut raw = self.raw.write().unwrap();
        if raw.time_left.is_running() {
            raw.time_left = raw.time_left.stop();
            info!("Timer stopped");
        }
    }

    pub fn match_abort(&self) {
        self.stop_timer()
        //TODO Other actions related to match abort
    }

    pub fn ds_mode(&self) -> enums::Mode {
        let raw = self.raw.read().unwrap();
        raw.ds_mode
    }

    pub fn set_ds_mode(&self, ds_mode: enums::Mode) {
        let mut raw = self.raw.write().unwrap();
        raw.ds_mode = ds_mode;
        info!("DS Mode set to {}", ds_mode);
    }

    pub fn is_safe(&self) -> bool {
        let raw = self.raw.read().unwrap();
        raw.is_safe
    }

    pub fn set_is_safe(&self, is_safe: bool) {
        let mut raw = self.raw.write().unwrap();
        raw.is_safe = is_safe;
        info!("Field safe flag set to {}", is_safe);
    }

    // Internal API -->

    pub(super) fn new() -> Self {
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
            udp_online: false,
            tcp_online: false,
        };

        let field = Self {
            raw: Arc::new(RwLock::new(field)),
        };

        field.driverstations().set_field(field.clone()).unwrap();

        field
    }

    pub(super) async fn run(
        &self,
        ds_address: IpAddr,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let mut tasks = JoinSet::new();

        let udp_address = SocketAddr::new(ds_address, 1160);
        let tcp_address = SocketAddr::new(ds_address, 1750);

        tasks.build_task().name("Field TCP Listener").spawn(
            self.clone().listen_for_tcp_connections_with_retry_loop(
                tcp_address,
                cancellation_token.clone(),
            ),
        )?;

        tasks.build_task().name("Field UDP Listener").spawn(
            self.clone()
                .listen_for_udp_messages_with_retry_loop(udp_address, cancellation_token.clone()),
        )?;

        tasks
            .build_task()
            .name("Field tick loop")
            .spawn(self.clone().tick_loop(cancellation_token.clone()))?;

        let run_field_tasks = async {
            while let Some(res) = tasks.join_next().await {
                res.context("Field tasks stopped unexpectedly")??;
            }
            anyhow::Ok(())
        };

        let driverstations = self.driverstations();

        let res = tokio::try_join!(
            run_field_tasks,
            driverstations.run(cancellation_token.clone())
        );

        if let Err(e) = res {
            return Err(e.context("Field run terminated unexpectedly"));
        }

        Ok(())
    }

    async fn listen_for_udp_messages_with_retry_loop(
        self,
        addr: SocketAddr,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let listen_with_retry_loop = async {
            loop {
                //Retry Loop
                let socket = UdpSocket::bind(addr)
                    .await
                    .context(new_bind_err("UDP", addr));
                if socket.is_err() {
                    error!("{}", socket.err().unwrap());
                    {
                        let mut raw_field = self.raw.write().unwrap();
                        raw_field.udp_online = false;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                    continue;
                }
                let socket = socket.unwrap();
                {
                    let mut raw_field = self.raw.write().unwrap();
                    raw_field.udp_online = false;
                }
                let driverstations = self.driverstations();

                let mut buf = vec![0; 1024];
                info!("Listening for UDP messages on {}", addr);
                loop {
                    match socket.recv_from(&mut buf).await {
                        Ok((size, _)) => {
                            if let Err(e) = driverstations
                                .decode_udp_message(buf[..size].to_vec())
                                .await
                                && e.to_string() != "unexpected end of file"
                            {
                                error!("Error decoding UDP message: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Error when reading UDP Message: {}", e);
                        }
                    }
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => Ok(()),
            _ = listen_with_retry_loop => Err(anyhow::anyhow!("UDP Listener closed unexpectedly")),
        }
    }

    async fn listen_for_tcp_connections_with_retry_loop(
        self,
        addr: SocketAddr,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<()> {
        let listen_with_retry_loop = async {
            loop {
                //Retry Loop
                let listener = TcpListener::bind(addr)
                    .await
                    .context(new_bind_err("TCP", addr));
                if listener.is_err() {
                    error!("{}", listener.err().unwrap());
                    {
                        let mut raw_field = self.raw.write().unwrap();
                        raw_field.tcp_online = false;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                    continue;
                }
                let listener = listener.unwrap();
                {
                    let mut raw_field = self.raw.write().unwrap();
                    raw_field.tcp_online = true;
                }
                let driverstations = self.driverstations();

                info!("Listening for TCP connections on {}", addr);
                loop {
                    match listener.accept().await {
                        Ok((stream, socket)) => {
                            if let Err(e) = driverstations
                                .handle_tcp_stream(stream, socket.ip(), cancellation_token.clone())
                                .await
                            {
                                error!("Error accepting TCP stream: {}", e);
                            }
                        }
                        Err(e) => {
                            error!("Error when accepting TCP Connection: {}", e);
                        }
                    }
                }
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => Ok(()),
            _ = listen_with_retry_loop => Err(anyhow::anyhow!("TCP Listener closed unexpectedly")),
        }
    }

    async fn tick_loop(self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(250));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        let interval_tick_loop = async {
            loop {
                interval.tick().await;
                self.tick();
            }
        };

        tokio::select! {
            _ = cancellation_token.cancelled() => Ok(()),
            _ = interval_tick_loop => Err(anyhow::anyhow!("Tick loop closed unexpectedly")),
        }
    }

    fn tick(&self) {
        // Respond to active faults
        if self
            .alarm_handler()
            .is_target_faulted(self.alarm_target().as_str())
        {
            self.match_abort();
        }
    }
}

fn new_bind_err(conn_type: &str, addr: SocketAddr) -> String {
    format!(
        "Coult not bind to {} {}. The host device may not have an interface with that address. To change the ds address, use the --ds-address option. Attempting bind again in 15 seconds.",
        conn_type, addr
    )
}
