use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

#[must_use]
pub fn get_subscriber(level: &str) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "{}={level},tower_http=debug,axum::rejection=trace",
            env!("CARGO_CRATE_NAME")
        )
    }));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
}

/// # Panics
///
/// This panics if the `LogTracer` cannot be initialized or `subscriber` cannot be set as global
/// default.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
