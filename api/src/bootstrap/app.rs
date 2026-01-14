use std::sync::Arc;

use super::config::{self, AppConfig};
use super::services::{self, AppService};

#[derive(Debug, Clone)]
pub struct App {
    config: Arc<config::AppConfig>,
    service: Arc<services::AppService>,
}

impl App {
    pub fn new(config: AppConfig, service: AppService) -> Self {
        Self {
            config: Arc::new(config),
            service: Arc::new(service),
        }
    }
}

#[tracing::instrument(
    name = "app.initialize",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub async fn init() -> App {
    let config = config::init()
        .inspect_err(|err| tracing::error!(error = ?err))
        .unwrap();

    let service = services::init(config.name.clone()).await;

    tracing::info!(
        "{} v{} initialized - running in {} ({:?}) mode",
        config.name,
        config.version,
        config.app_env,
        config.profile
    );

    App::new(config, service)
}
