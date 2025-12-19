use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

const ENV_TELEMETRY_LOGS_EXPORTER: &'static str = "TELEMETRY_LOGS_EXPORTER";
const DEFAULT_LOG_LEVEL: &'static str = "trace";

pub fn logs_init(
    app_name: &str,
    app_version: &str,
    app_env: &str,
) -> Result<(), tracing_loki::Error> {
    let logs_exporter_url = std::env::var(ENV_TELEMETRY_LOGS_EXPORTER)
        .expect("missing required environment variable: `TELEMETRY_LOGS_EXPORTER`");

    let url = url::Url::parse(&logs_exporter_url)
        .expect("Failed to parse the URL: `TELEMETRY_LOGS_EXPORTER` environment variable value");

    let (layer, task) = tracing_loki::builder()
        .label("service", app_name)?
        .label("version", app_version)?
        .label("env", app_env)?
        .build_url(url)?;

    // use `RUST_LOG` if available, otherwise default to [DEFAULT_LOG_LEVEL]
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layer)
        .init();

    tokio::spawn(task);

    tracing::info!(
        task = "tracing_setup",
        result = "success",
        "telemetry initialized"
    );

    Ok(())
}
