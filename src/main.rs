pub mod alarms;
pub mod authentication;
pub mod database;
pub mod difftimer;
pub mod field;
pub mod fmscore;
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
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets the address that the FMS listens to for driver stations.
    #[arg(long, default_value = "10.0.100.5", env = "NEVERMORE_DS_ADDRESS")]
    ds_address: IpAddr,

    /// Sets the listening address of the http server.
    #[arg(short = 'a', long, default_value = "0.0.0.0:8000", env = "NEVERMORE_WEB_ADDRESS")]
    web_address: SocketAddr,

    /// Sets the expected hostname for the http server
    #[arg(short = 'n', long, default_value = "10.0.100.5:8000", env = "NEVERMORE_WEB_HOSTNAME")]
    web_hostname: String,

    /// Enables tls for the http server
    #[arg(long, env = "NEVERMORE_WEB_TLS")]
    web_tls: bool,

    #[arg(short, long, default_value = "info", env = "NEVERMORE_LOG")]
    log_level: String,

    /// Set a custom data directory.
    #[arg(long, env = "NEVERMORE_DATA_DIR")]
    data_dir: Option<std::path::PathBuf>,

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

    let args = Args::parse();

    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_filters(args.log_level.as_str())
        .init();

    info!("{BIRD}");

    info!("Starting {NAME} v{VERSION} by {AUTHORS}...");

    let fms_core = FMSCore::new(args.data_dir, args.web_hostname, args.web_tls)?;

    let cancellation_token = CancellationToken::new();

    let res = fms_core
        .run(args.ds_address, args.web_address, cancellation_token.clone())
        .await;

    if let Err(e) = res {
        return Err(e.context("Main process terminated unexpectedly"));
    }

    Ok(())
}
