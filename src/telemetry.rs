use opentelemetry::global;
use opentelemetry_jaeger::Uninstall;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// TODO: don't pass `Uninstall` around...
pub fn get_subscriber(name: &str, env_filter: &str) -> (impl Subscriber + Send + Sync, Uninstall) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name.to_string(), std::io::stdout);

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let (tracer, uninstall) = opentelemetry_jaeger::new_pipeline()
        .with_service_name(name)
        .install()
        .expect("cannot install jaeger pipeline");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    (
        Registry::default()
            .with(telemetry)
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer),
        uninstall,
    )
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}
