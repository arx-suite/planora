use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ENV_FILTER: &'static str = "info,arx_gatehouse=trace";

pub fn tracing_init() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_ENV_FILTER));

    let fmt_layer = fmt::layer().with_target(true).with_level(true).compact();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}
