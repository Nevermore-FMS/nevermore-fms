extern crate anyhow;
extern crate async_trait;
extern crate clap;
extern crate log;

pub mod alarms;
pub mod difftimer;
pub mod field;
pub mod game;
pub mod graph;
pub mod web;

use clap::{Parser, ValueEnum};
use log::*;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use crate::field::Field;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const BIRD: &'static str = include_str!("eaobird.txt");

#[derive(ValueEnum, PartialEq, Debug, Clone)]
pub enum UIWindow {
    Admin,
}

/// An alternative FIRST FMS designed around extensibility and compatibility.
#[derive(Parser, Clone)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets the address that the FMS listens to for driver stations.
    #[clap(long, default_value = "10.0.100.5", env = "NEVERMORE_DS_ADDRESS")]
    ds_address: IpAddr,

    /// Sets the listening address of the http server.
    #[clap(long, default_value = "0.0.0.0:8000", env = "NEVERMORE_WEB_ADDRESS")]
    web_address: SocketAddr,

    #[clap(short, long)]
    tray: bool,

    // Opens only a specific window on startup, and stops once that window is closed.
    #[clap(value_enum, short, long, env = "NEVERMORE_UI_WINDOW")]
    window: Option<UIWindow>,

    // Opens the window in fullscreen.
    #[clap(short, long)]
    fullscreen: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    console_subscriber::init();

    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_filters(&env::var("NEVERMORE_LOG").unwrap_or(String::from("info")))
        .init();

    info!("{}", BIRD);

    let cli = Cli::parse();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let field = Field::new(cli.ds_address).await?;

    field
        .driverstations()
        .await
        .add_driverstation(5276, field::enums::AllianceStation::Red1)
        .await
        .unwrap()
        .update_expected_ip("0.0.0.0/0".parse().unwrap())
        .await; //TODO Remove

    web::start_server(cli.web_address, field.clone()).await;

    return Ok(());
}
