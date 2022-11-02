pub mod application;
pub mod control;
pub mod field;
pub mod plugin;
pub mod difftimer;
pub mod web;
pub mod store;
mod ui;

use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use anyhow::Context;
use clap::{Parser, ArgEnum};
use log::*;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const BIRD: &'static str = include_str!("eaobird.txt");

#[derive(ArgEnum, PartialEq, Debug, Clone)]
pub enum UIWindow {
    Admin
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
    #[clap(arg_enum, short, long, env = "NEVERMORE_UI_WINDOW")]
    window: Option<UIWindow>,

    // Opens the window in fullscreen.
    #[clap(short, long)]
    fullscreen: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_filters(&env::var("NEVERMORE_LOG").unwrap_or(String::from("info")))
        .init();

    info!("{}", BIRD);

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let cli = Cli::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let http_addr: SocketAddr = cli.web_address;

    let mut window = cli.window.clone();
    if let Some(window) = window.take() {
        rt.spawn(async_main(cli.clone()));

        ui::create_window(window, http_addr, cli.fullscreen)?;
    } else {
        if cli.tray {
            rt.spawn(async_main(cli.clone()));

            ui::create_tray(http_addr, cli.fullscreen)?;
        } else {
            rt.block_on(async_main(cli))?;
        };
    }

    return Ok(());
}

async fn async_main(cli: Cli) -> anyhow::Result<()> {
    let application = application::Application::new(cli.ds_address, cli.web_address)
        .await
        .context("Error while creating application, couldn't start Nevermore")?;

    application.field().await.control_system().await.register_plugin(String::from("defaultplugin")).await; //TODO Remove defaultplugin

    application.wait_for_terminate().await;
    
    return Ok(());
}
