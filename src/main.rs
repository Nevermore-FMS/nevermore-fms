mod game;
mod field;

#[macro_use]
extern crate log;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let application = field::Field::new("nevermore".to_string()).await?;

    loop {

    }

    /*let mut game_engine = DenoGameEngine::new()?;
    game_engine
        .start(String::from(include_str!("test.js")))
        .await?;*/

    Ok(())
}
