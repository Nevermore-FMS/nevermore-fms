pub mod application;
pub mod database;
pub mod field;
pub mod game;
pub mod http;
pub mod pub_sub;

use log::info;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    info!("Starting {} v{} by {}...", NAME, VERSION, AUTHORS);

    let app = application::Application::new().await?;

    http::start(app).await;

    Ok(())
}
