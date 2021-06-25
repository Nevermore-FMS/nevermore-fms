mod nevermore;
mod v8;
mod robot;

#[macro_use] extern crate log;

use log::LevelFilter;
use crate::nevermore::Nevermore;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let mut application = nevermore::Nevermore::new().await?;
    application.start().await;
    Ok(())
}