use std::{env, str::FromStr};

// environment variables
#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("missing required environment variable `{0}`")]
    Missing(String),

    #[error("invalid value for `{key}`: {reason}")]
    Invalid { key: String, reason: String },
}

pub trait EnvParse: Sized {
    fn parse_env(key: &str, value: String) -> Result<Self, EnvError>;
}

pub fn get_env<T: EnvParse>(key: &str) -> Result<T, EnvError> {
    let value = env::var(key).map_err(|_| EnvError::Missing(key.into()))?;
    T::parse_env(key, value)
}

impl<T> EnvParse for T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn parse_env(key: &str, value: String) -> Result<Self, EnvError> {
        value.parse::<T>().map_err(|e| EnvError::Invalid {
            key: key.into(),
            reason: e.to_string(),
        })
    }
}

// time
pub fn current_utc_timestamp() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
