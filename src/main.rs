pub mod application;
pub mod field;
pub mod http;
pub mod models;
pub mod pub_sub;
pub mod worker;

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
