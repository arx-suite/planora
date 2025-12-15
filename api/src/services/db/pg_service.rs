use sqlx::{PgPool, postgres::PgPoolOptions};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::common::ApiError;

const DEFAULT_INIT_POOL: &'static str = "planora";
const DEFAULT_MAX_CONNECTIONS: u32 = 5;
const ENV_PG_DATABASE_URL: &'static str = "PG_DATABASE_URL";

#[derive(Debug, Clone)]
pub struct DbManager {
    pools: Arc<RwLock<HashMap<String, PgPool>>>,
}

impl DbManager {
    pub const PLANORA_POOL: &'static str = "planora";

    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn init_pool(&self) -> sqlx::Result<()> {
        let url = std::env::var(ENV_PG_DATABASE_URL)
            .expect("missing required environment variable: PG_DATABASE_URL");
        let name = DEFAULT_INIT_POOL;

        let safe_url = match url.split('@').nth(1) {
            Some(host_part) => host_part,
            None => "unknown-host",
        };
        tracing::debug!(db_name = %name, host = %safe_url, "attempting to connect to PostgreSQL");

        match PgPoolOptions::new()
            .max_connections(DEFAULT_MAX_CONNECTIONS)
            .connect(&url)
            .await
        {
            Ok(pool) => {
                let mut map = self.pools.write().await;
                map.insert(name.to_string(), pool);
                tracing::debug!(db_name = %name, "PostgreSQL connection successfully established");
                Ok(())
            }
            Err(e) => {
                tracing::error!(db_name = %name, error = %e, "failed to establish PostgreSQL connection");
                Err(e)
            }
        }
    }

    pub async fn get_pool(&self, name: &str) -> Result<PgPool, ApiError> {
        let map = self.pools.read().await;

        match map.get(name).cloned() {
            Some(pool) => Ok(pool),
            None => {
                tracing::error!("failed to get database pool {name}");
                return Err(ApiError::Internal("internal error".to_string()));
            }
        }
    }

    #[inline]
    pub async fn get_planora_pool(&self) -> Result<PgPool, ApiError> {
        self.get_pool(Self::PLANORA_POOL).await
    }

    pub async fn close_all(&self) {
        let mut map = self.pools.write().await;
        for (_, pool) in map.drain() {
            pool.close().await;
        }
    }
}
