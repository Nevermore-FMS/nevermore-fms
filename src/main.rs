pub mod application;
pub mod field;
pub mod control;
pub mod plugin;

use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use anyhow::Context;
use clap::Parser;
use log::*;

use crate::field::{driverstation::DriverStation, enums::AllianceStation};

const BIRD: &'static str = include_str!("eaobird.txt");

/// An alternative FIRST FMS designed around extensibility and compatibility.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets the uri used to access the SQL Database using sqlx. TODO
    #[clap(long, default_value = "file:main.db", env = "NEVERMORE_DB_URI")]
    db_uri: String,

    /// Sets the address that the FMS listens to for driver stations.
    #[clap(long, default_value = "10.0.100.5", env = "NEVERMORE_DS_ADDRESS")]
    ds_address: IpAddr,

    /// Sets the listening address of the http server.
    #[clap(long, default_value = "0.0.0.0:8000", env = "NEVERMORE_WEB_ADDRESS")]
    web_address: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_filters(&env::var("NEVERMORE_LOG").unwrap_or(String::from("info")))
        .init();

    info!("{}", BIRD);

    let cli = Cli::parse();

    let application = application::Application::new(None, cli.ds_address)
        .await
        .context("Error while creating application, couldn't start Nevermore")?;

    //TODO Remove - Debug
    application
        .field()
        .await
        .driverstations()
        .await
        .add_driverstation(DriverStation::new(5276, AllianceStation::Red1, "0.0.0.0/0".parse().unwrap())).await?;

    application.wait_for_terminate().await;

    return Ok(());
}
