#![allow(dead_code)]

use config::AppConfig;
use std::sync::Arc;

mod config;
mod services;

pub use services::*;

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
