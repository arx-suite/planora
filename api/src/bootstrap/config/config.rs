use super::profile::Profile;
use crate::common::utils::{self, EnvError};

const CONFIG_FILE: &str = ".env.local";
const DEFAULT_APP_ENVIRONMENT: &str = "development";

const ENV_APP_ENVIRONMENT: &str = "APP_ENV";
const ENV_API_HOST: &str = "API_HOST";
const ENV_API_PORT: &str = "API_PORT";
const ENV_WEB_BASE_URL: &str = "WEB_BASE_URL";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub profile: Profile,
    pub app_env: String,
    pub host: String,
    pub port: u16,
    pub web_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum AppConfigError {
    #[error("invalid or missing environment configuration")]
    Environment(#[from] EnvError),

    #[error("production environment requires a release build (--release)")]
    ReleaseBuildRequired,

    #[error("failed to load environment configuration file")]
    ConfigFileLoad(#[from] dotenvy::Error),
}

type AppConfigResult = std::result::Result<AppConfig, AppConfigError>;

impl AppConfig {
    pub fn new(
        profile: Profile,
        host: String,
        port: u16,
        web_url: String,
        app_env: String,
    ) -> AppConfigResult {
        let name = env!("CARGO_PKG_NAME").to_owned();
        let version = env!("CARGO_PKG_VERSION").to_owned();

        Ok(Self {
            name,
            version,
            profile: profile.to_owned(),
            app_env,
            host,
            port,
            web_url,
        })
    }

    #[inline]
    pub fn from_env() -> AppConfigResult {
        let app_env =
            utils::get_env::<String>(ENV_APP_ENVIRONMENT).unwrap_or(DEFAULT_APP_ENVIRONMENT.into());
        let is_prod = Self::is_production(&app_env);

        let profile = if cfg!(debug_assertions) {
            Profile::Development
        } else {
            Profile::Release
        };

        if is_prod && !profile.is_release() {
            return Err(AppConfigError::ReleaseBuildRequired);
        }

        // Load development-specific configuration
        if !is_prod {
            dotenvy::from_filename(CONFIG_FILE)?;
        }

        let host = utils::get_env::<String>(ENV_API_HOST)?;
        let port = utils::get_env::<u16>(ENV_API_PORT)?;
        let web_base_url = utils::get_env::<url::Url>(ENV_WEB_BASE_URL)?.to_string();

        Self::new(profile, host, port, web_base_url, app_env)
    }

    #[inline]
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[inline]
    fn is_production(app_env: &str) -> bool {
        matches!(app_env, "production")
    }

    #[inline]
    pub fn is_production_env(&self) -> bool {
        Self::is_production(&self.app_env)
    }
}

#[tracing::instrument(
    name = "config.main",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub fn init() -> AppConfigResult {
    AppConfig::from_env()
}
