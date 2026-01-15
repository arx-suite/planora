#![allow(dead_code)]

use config::AppConfig;
use services::{AppService, AuthService, CacheService, DbService, MailService, S3Service};
use std::sync::Arc;
use telemetry::telemetry::ObservabilityGuard;

pub mod config;
pub mod services;
pub mod telemetry;

#[derive(Debug, Clone)]
pub struct App {
    config: Arc<AppConfig>,
    services: Arc<AppService>,
}

impl App {
    pub fn new(config: AppConfig, service: AppService) -> Self {
        Self {
            config: Arc::new(config),
            services: Arc::new(service),
        }
    }

    pub fn config(&self) -> &AppConfig {
        self.config.as_ref()
    }

    #[inline]
    pub fn auth(&self) -> &AuthService {
        &self.services.auth
    }

    #[inline]
    pub fn db(&self) -> &DbService {
        &self.services.db
    }

    #[inline]
    pub fn cache(&self) -> &CacheService {
        &self.services.cache
    }

    #[inline]
    pub fn s3(&self) -> &S3Service {
        &self.services.s3
    }

    #[inline]
    pub fn mail(&self) -> &MailService {
        &self.services.mail
    }
}

#[tracing::instrument(
    name = "app.initialize",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub async fn init() -> (App, ObservabilityGuard) {
    // app configuration
    let config = config::init()
        .inspect_err(|err| tracing::error!(error = ?err))
        .unwrap();

    // telemetry
    let guard = telemetry::telemetry::init();

    // services
    let service = services::init(config.name.clone()).await;

    tracing::info!(
        "{} v{} initialized - running in {} ({:?}) mode",
        config.name,
        config.version,
        config.app_env,
        config.profile
    );

    (App::new(config, service), guard)
}
