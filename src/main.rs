pub mod application;
pub mod field;
pub mod http;
pub mod models;
pub mod plugin;
pub mod pub_sub;
pub mod session;
pub mod ui;

use std::{net::SocketAddr};

use clap::{AppSettings, ArgEnum, Clap};
use log::info;

#[cfg(feature = "developer")]
use log::warn;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const BIRD: &'static str = include_str!("eaobird.txt");
#[cfg(feature = "developer")]
const DEV_MESSAGE: &'static str = "Development Mode is enabled. Plugins can be modified remotely without authentication, DO NOT USE THIS IN PRODUCTION.";

#[derive(ArgEnum, PartialEq, Debug, Clone)]
pub enum UIWindow {
    Admin,
    Devtools,
    GraphqlPlayground,
    RefereePanel,
}

/// An alternative FIRST FMS designed around extensibility and compatibility.
#[derive(Clap, Clone)]
#[clap(version = VERSION, author = AUTHORS)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the uri used to access the SQL Database using sqlx.
    #[clap(short, long, default_value = "main.db", env = "NEVERMORE_DB_URI")]
    db_uri: String,

    /// Sets the address that the FMS listens to for driver stations.
    #[clap(long, default_value = "10.0.100.5", env = "NEVERMORE_DS_ADDRESS")]
    ds_address: String,

    /// Sets the listening address of the http server.
    #[clap(
        short,
        long,
        default_value = "0.0.0.0:8000",
        env = "NEVERMORE_LISTEN_ADDR"
    )]
    listen_addr: String,

    // Defines whether a webview and tray should be created.
    #[clap(short, long)]
    tray: bool,

    // Opens only a specific window on startup, and stops once that window is closed.
    #[clap(arg_enum, short, long, env = "NEVERMORE_UI_WINDOW")]
    window: Option<UIWindow>,

    // Opens the window in fullscreen.
    #[clap(short, long)]
    fullscreen: bool,
}

fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_ok() {
        pretty_env_logger::try_init()?;
    } else {
        pretty_env_logger::formatted_timed_builder()
            .filter_level(log::LevelFilter::Warn)
            .filter_level(log::LevelFilter::Info)
            .try_init()?;
    }

    let opts = Opts::parse();

    info!("{}", BIRD);

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    #[cfg(feature = "developer")]
    warn!("{}", DEV_MESSAGE);

    let rt = tokio::runtime::Runtime::new().unwrap();

    let http_addr: SocketAddr = opts.listen_addr.parse()?;

    let mut window = opts.window.clone();
    if let Some(window) = window.take() {
        rt.spawn(async_main(opts.clone(), http_addr.clone()));

        ui::create_window(window, http_addr, opts.fullscreen)?;
    } else {
        if opts.tray {
            rt.spawn(async_main(opts.clone(), http_addr.clone()));

            ui::create_tray(http_addr, opts.fullscreen)?;
        } else {
            rt.block_on(async_main(opts, http_addr));
        };
    }

    Ok(())
}

// Starts all Tokio based services.
async fn async_main(opts: Opts, http_addr: SocketAddr) {
    let app = application::Application::new(Some(opts.db_uri), opts.ds_address).await
    .expect("Error while creating application, couldn't start Nevermore");
    http::start(app, http_addr).await;
}
