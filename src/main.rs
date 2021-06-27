mod game;
mod nevermore;
mod robot;

#[macro_use]
extern crate log;

use crate::game::DenoGameEngine;
use crate::nevermore::Nevermore;
use log::LevelFilter;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    //let mut application = nevermore::Nevermore::new().await?;
    //application.lock().await.start().await;

    let mut game_engine = DenoGameEngine::new()?;
    game_engine
        .start(String::from(include_str!("test.js")))
        .await?;

    Ok(())
}
