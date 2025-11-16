use hoc::{config::Settings, telemetry};

use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info, instrument};

fn init() -> Result<()> {
    dotenvy::dotenv().ok();

    telemetry::init_subscriber(telemetry::get_subscriber("info"))
}

#[tokio::main]
#[instrument(skip_all, fields(version = env!("CARGO_PKG_VERSION")))]
async fn main() -> Result<()> {
    init()?;

    let settings = Settings::load()?;

    let address = format!("{}:{}", settings.host, settings.port);
    info!(?settings, "starting server");
    let listener = TcpListener::bind(address).await?;
    hoc::run(listener, settings).await?;
    Ok(())
}
