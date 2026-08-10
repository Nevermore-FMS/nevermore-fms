pub mod alarms;
pub mod database;
pub mod difftimer;
pub mod field;
pub mod fmscore;
pub mod graph;
pub mod web;
// TODO These do not need to be pub

use clap::{Parser, ValueEnum};
use log::info;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};
use tokio_util::sync::CancellationToken;

use crate::fmscore::FMSCore;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const BIRD: &str = include_str!("../assets/nevermorebird.txt");

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

    #[clap(short, long, default_value = "info", env = "NEVERMORE_LOG")]
    log_level: String,

    /// Set a custom data directory.
    #[clap(long, env = "NEVERMORE_DATA_DIR")]
    data_dir: Option<std::path::PathBuf>,

    #[clap(short, long)]
    tray: bool,

    /// Opens only a specific window on startup, and stops once that window is closed.
    #[clap(value_enum, short, long, env = "NEVERMORE_UI_WINDOW")]
    window: Option<UIWindow>,

    /// Opens the window in fullscreen.
    #[clap(short, long)]
    fullscreen: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    console_subscriber::init();

    let cli = Cli::parse();

    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_filters(cli.log_level.as_str())
        .init();

    info!("{BIRD}");

    info!("Starting {NAME} v{VERSION} by {AUTHORS}...");

    let fms_core = FMSCore::new(cli.data_dir)?;

    let cancellation_token = CancellationToken::new();

    let res = fms_core
        .run(cli.ds_address, cli.web_address, cancellation_token.clone())
        .await;

    if let Err(e) = res {
        return Err(e.context("Main process terminated unexpectedly"));
    }

    Ok(())
}
