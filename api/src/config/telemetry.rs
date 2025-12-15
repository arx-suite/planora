use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_TRACE_LEVEL: &'static str = "debug";

pub fn init() {
    // use `RUST_LOG` if available, otherwise default to `TRACE`
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_TRACE_LEVEL));

    let fmt_layer = fmt::layer().with_target(true).with_level(true).compact();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("telemetry initialized");
}
