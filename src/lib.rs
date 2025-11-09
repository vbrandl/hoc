mod cache;
pub mod config;
pub mod count;
mod error;
mod hoc;
pub mod http;
mod platform;
mod statics;
pub mod telemetry;
mod template;

use crate::config::Settings;

use tokio::{net::TcpListener, signal};
use tracing::{Instrument, info, info_span};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

async fn start_server(listener: TcpListener, settings: Settings) -> std::io::Result<()> {
    axum::serve(listener, http::router(settings))
        .with_graceful_shutdown(shutdown_signal())
        .await
}

/// Start the server.
///
/// # Errors
///
/// * server cannot bind to `listener`
pub async fn run(listener: TcpListener, settings: Settings) -> std::io::Result<()> {
    let span = info_span!("hoc", version = env!("CARGO_PKG_VERSION"));
    let _ = span.enter();
    start_server(listener, settings).instrument(span).await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
