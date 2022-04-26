pub mod connection;
pub mod driverstation;
pub mod enums;

use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc, collections::HashMap, hash::Hash,
};

use anyhow::Context;
use log::*;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{
        broadcast::{self},
        RwLock,
    },
};

use crate::control::{enabler::SyncEnabler, estopper::SyncEstopper, ControlSystem};

use self::{driverstation::DriverStations, enums::TournamentLevel};

struct RawField {
    event_name: String,
    tournament_level: TournamentLevel,
    match_number: u16,
    play_number: u8,
    time_left: f64,
    driverstations: DriverStations,
    terminate_signal: Option<broadcast::Sender<()>>,
    running_signal: async_channel::Receiver<()>,
    control_system: ControlSystem,
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

    pub async fn driverstations(&self) -> DriverStations {
        let raw = self.raw.read().await;
        raw.driverstations.clone()
    }

    pub async fn event_name(&self) -> String {
        let raw = self.raw.read().await;
        raw.event_name.clone()
    }

    pub async fn tournament_level(&self) -> TournamentLevel {
        let raw = self.raw.read().await;
        raw.tournament_level.clone()
    }

    pub async fn match_number(&self) -> u16 {
        let raw = self.raw.read().await;
        raw.match_number.clone()
    }

    pub async fn play_number(&self) -> u8 {
        let raw = self.raw.read().await;
        raw.play_number.clone()
    }

    pub async fn time_remaining(&self) -> f64 {
        let raw = self.raw.read().await;
        raw.time_left
    }

    pub async fn control_system(&self) -> ControlSystem {
        let raw = self.raw.read().await;
        raw.control_system.clone()
    }

    // Internal API -->

    pub(super) async fn new(event_name: String, ds_address: IpAddr) -> anyhow::Result<Self> {
        let (terminate_sender, _) = broadcast::channel(1);

        let (indicate_running, running_signal) = async_channel::bounded(1);

        let field = RawField {
            event_name,
            tournament_level: TournamentLevel::Test,
            match_number: 0,
            play_number: 0,
            time_left: 0.0,
            driverstations: DriverStations::new(None),
            terminate_signal: Some(terminate_sender),
            running_signal,
            control_system: ControlSystem::new(),
            udp_online: false,
            tcp_online: false,
        };

        let field = Self {
            raw: Arc::new(RwLock::new(field)),
        };
        
        field.driverstations().await.set_field(field.clone()).await?;

        let udp_address = SocketAddr::new(ds_address, 1160);
        let tcp_address = SocketAddr::new(ds_address, 1750);
        let async_field = field.clone();
        tokio::spawn(async move {
            let (udp_result, tcp_result) = tokio::join!(
                async_field.listen_for_udp_messages(udp_address),
                async_field.listen_for_tcp_connections(tcp_address)
            );
            udp_result.unwrap();
            tcp_result.unwrap();
            drop(indicate_running);
        });

        Ok(field)
    }

    async fn listen_for_udp_messages(&self, addr: SocketAddr) -> anyhow::Result<()> {
        loop {
            let mut raw_field = self.raw.write().await;
            let socket = UdpSocket::bind(addr).await.context(bind_err(addr));
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
            drop(raw_field);

            let mut buf = vec![0; 1024];
            info!("Listening for UDP messages on {}", addr);
            loop {
                tokio::select! {
                    result = socket.recv_from(&mut buf) => {
                        match result {
                            Ok((size, _)) => {
                                let raw_field = self.raw.read().await;
                                if let Err(e) = raw_field.driverstations.decode_udp_message(buf[..size].to_vec()).await {
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
                        info!("Closing the UDP listener.");
                        return Ok(());
                    }
                }
            }
        }
    }

    async fn listen_for_tcp_connections(&self, addr: SocketAddr) -> anyhow::Result<()> {
        loop {
            let mut raw_field = self.raw.write().await;
            let listener = TcpListener::bind(addr).await.context(bind_err(addr));
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
            drop(raw_field);

            info!("Listening for TCP connections on {}", addr);
            loop {
                tokio::select! {
                    socket = listener.accept() => {
                        match socket {
                            Ok((stream, socket)) => {
                                let raw_field = self.raw.read().await;
                                if let Err(e) = raw_field.driverstations.handle_tcp_stream(stream, socket.ip()).await {
                                    error!("Error accepting TCP stream: {}", e);
                                }
                            },
                            Err(e) => {
                                error!("Error when accepting TCP Connection: {}", e);
                            }
                        }
                    }
                    _ = term_rx.recv() => {
                        info!("Closing the TCP listener.");
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn bind_err(addr: SocketAddr) -> String {
    format!("Coult not bind to {}. This computer may not have an interface with that address. To change the ds address, use the --ds-address option. Attempting bind again in 15 seconds.", addr)
}
