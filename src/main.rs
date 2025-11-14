use hoc::{config::Settings, telemetry};

use anyhow::Result;

fn init() {
    dotenvy::dotenv().ok();

    telemetry::init_subscriber(telemetry::get_subscriber("info"));
}

#[tokio::main]
async fn main() -> Result<()> {
    init();

    let settings = Settings::load()?;

    let listener = settings.listener().await?;

    hoc::run(listener, settings).await?;
    Ok(())
}
