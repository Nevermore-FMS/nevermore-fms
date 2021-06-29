pub mod field;
pub mod game;
pub mod pub_sub;
pub mod graph;

#[macro_use]
extern crate log;

use crate::field::enums::AllianceStation;
use crate::game::DenoGameEngine;


use tokio::io::{AsyncBufReadExt, BufReader};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let field = field::Field::new("nevermore".to_string()).await?;

    let pubsub = pub_sub::PubSub::new();

    let mut game_engine = DenoGameEngine::new(field.clone(), pubsub.clone())?;
    game_engine.run_code("main".to_string(), String::from(include_str!("test.js")))?;
    game_engine.run_event_loop().await?;

    graph::start().await;

    Ok(())
}
