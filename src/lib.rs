pub mod cache;
pub mod config;
pub mod count;
mod error;
mod hoc;
pub mod http;
mod platform;
mod statics;
pub mod telemetry;
mod template;
pub mod worker;

use std::sync::{Arc, atomic::AtomicUsize};

use crate::{
    cache::Persist, config::Settings, count::count_repositories, error::Result, http::AppState,
    worker::Queue,
};

use tokio::{net::TcpListener, signal};
use tracing::info;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

async fn start_server(listener: TcpListener, settings: Settings) -> Result<()> {
    let queue = Queue::new();
    let cache = Persist::new(settings.clone());
    let repo_count = AtomicUsize::new(count_repositories(&settings.repodir)?);
    let state = Arc::new(AppState {
        settings,
        repo_count,
        cache,
        queue,
    });
    let router = http::router(state.clone());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    state.queue.close();
    Ok(())
}

/// Start the server.
///
/// # Errors
///
/// * server cannot bind to `listener`
pub async fn run(listener: TcpListener, settings: Settings) -> Result<()> {
    start_server(listener, settings).await
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
