use std::env;

const CONFIG_FILE: &'static str = ".env.local";
const KEY_APP_ENVIRONMENT: &'static str = "APP_ENV";
const KEY_SERVER_HOST: &'static str = "SERVER_HOST";
const KEY_SERVER_PORT: &'static str = "SERVER_PORT";
const KEY_NEXT_BASE_URL: &'static str = "NEXT_PUBLIC_BASE_URL";

#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_version: String,
    pub profile: String,
    pub host: String,
    pub port: u16,
    pub next_base_url: String,
    pub app_env: String,
}

impl Config {
    pub fn from_env() -> Self {
        let app_name = env!("CARGO_PKG_NAME").to_owned();
        let app_version = env!("CARGO_PKG_VERSION").to_owned();

        let profile = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        let app_env = env::var(KEY_APP_ENVIRONMENT).unwrap_or("development".into());
        let is_prod = Self::is_production(&app_env);

        tracing::info!("setting up configuration for `{app_env}` environment");

        if is_prod && profile != "release" {
            tracing::error!("production environment requires a release build");
            panic!("profile should be release");
        }

        // Load development-specific configuration
        if !is_prod {
            if let Err(err) = dotenvy::from_filename(CONFIG_FILE) {
                tracing::trace!("could not load {CONFIG_FILE}: {err}");
            } else {
                tracing::trace!("loaded {CONFIG_FILE}");
            }
        }

        let host =
            env::var(KEY_SERVER_HOST).expect("missing required environment variable: SERVER_HOST");
        let port = env::var(KEY_SERVER_PORT)
            .expect("missing required environment variable: SERVER_PORT")
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid number");
        let next_base_url = env::var(KEY_NEXT_BASE_URL)
            .expect("missing required environment variable: NEXT_BASE_URL");

        Self {
            app_name,
            app_version,
            profile: profile.to_owned(),
            host,
            port,
            next_base_url,
            app_env,
        }
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
