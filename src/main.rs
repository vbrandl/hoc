use hoc::{config::Settings, telemetry};

use tokio::net::TcpListener;

fn init() {
    dotenvy::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    telemetry::init_subscriber(telemetry::get_subscriber("info"));
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init();

    // TODO: error handling
    let settings = Settings::load().expect("Cannot load config");

    let address = format!("{}:{}", settings.host, settings.port);
    // TODO: error handling
    let listener = TcpListener::bind(address).await?;
    hoc::run(listener, settings).await.expect("Server error");
    Ok(())
}
