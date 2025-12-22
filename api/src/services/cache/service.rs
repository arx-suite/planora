use std::time::Duration;

use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, Connection, Pool};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::CacheResult;

const ENV_REDIS_URL: &'static str = "REDIS_URL";

#[derive(Debug, Clone)]
pub struct CacheService {
    pool: Pool,
}

impl CacheService {
    fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn get_conn(&self) -> CacheResult<Connection> {
        let conn = self.pool.get().await?;
        Ok(conn)
    }

    pub async fn get<T>(&self, key: &str) -> CacheResult<Option<T>>
    where
        T: DeserializeOwned,
    {
        let mut conn = self.get_conn().await?;

        let value: Option<String> = conn.get(key.to_string()).await?;

        if let Some(json) = value {
            let value = serde_json::from_str(&json)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T) -> CacheResult<()>
    where
        T: Serialize,
    {
        let mut conn = self.get_conn().await?;
        let json = serde_json::to_string(value)?;
        conn.set::<_, _, String>(key, json).await?;
        Ok(())
    }

    pub async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: Duration) -> CacheResult<()>
    where
        T: Serialize,
    {
        let mut conn = self.get_conn().await?;
        let json = serde_json::to_string(value)?;
        let _: () = conn.set_ex(key, json, ttl.as_secs()).await?;
        Ok(())
    }

    pub async fn delete(&self, key: String) -> CacheResult<()> {
        let mut conn = self.get_conn().await?;
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn expire(&self, key: &str, ttl: Duration) -> CacheResult<()> {
        let mut conn = self.get_conn().await?;
        conn.expire::<_, ()>(key, ttl.as_secs() as i64).await?;
        Ok(())
    }
}

#[tracing::instrument(name = "service.cache", skip_all)]
pub fn init() -> CacheService {
    let url =
        std::env::var(ENV_REDIS_URL).expect("missing required environment variable: `REDIS_URL`");

    let cfg = Config::from_url(url);
    let pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();

    CacheService::new(pool)
}
