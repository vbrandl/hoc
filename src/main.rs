use hoc::{config::Settings, telemetry};

use std::net::TcpListener;

fn init() {
    dotenvy::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    telemetry::init_subscriber(telemetry::get_subscriber("hoc", "info"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init();

    // TODO: error handling
    let settings = Settings::load().expect("Cannot load config");

    let address = format!("{}:{}", settings.host, settings.port);
    // TODO: error handling
    let listener = TcpListener::bind(address)?;
    hoc::run(listener, settings)
        .await
        .expect("Server error")
        .await
}
