use hoc::{config::Settings, telemetry};

use anyhow::Result;

fn init() {
    dotenvy::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    telemetry::init_subscriber(telemetry::get_subscriber("info"));
}

#[tokio::main]
async fn main() -> Result<()> {
    init();

    // TODO: error handling
    let settings = Settings::load()?;

    let listener = settings.listener().await?;

    hoc::run(listener, settings).await?;
    Ok(())
}
