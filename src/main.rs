use hoc::{config::Settings, telemetry};
use std::net::TcpListener;

fn init() -> opentelemetry_jaeger::Uninstall {
    dotenv::dotenv().ok();
    openssl_probe::init_ssl_cert_env_vars();

    let (subscriber, uninstall) = telemetry::get_subscriber("hoc", "info");
    telemetry::init_subscriber(subscriber);
    uninstall
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _uninstall = init();

    // TODO: error handling
    let settings = Settings::load().expect("Cannot load config");

    let address = format!("{}:{}", settings.host, settings.port);
    // TODO: error handling
    let listener = TcpListener::bind(address).expect("cannot bind addres");

    hoc::run(listener, settings)
        .await
        .expect("Server error")
        .await
}
