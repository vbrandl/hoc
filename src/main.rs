use hoc::{config::Settings, telemetry};

use anyhow::Result;
use tokio::net::TcpListener;

fn init() -> Result<()> {
    dotenvy::dotenv().ok();

    telemetry::init_subscriber(telemetry::get_subscriber("info"))
}

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    let settings = Settings::load()?;

    let address = format!("{}:{}", settings.host, settings.port);
    let listener = TcpListener::bind(address).await?;
    hoc::run(listener, settings).await?;
    Ok(())
}
