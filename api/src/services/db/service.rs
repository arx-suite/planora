use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::collections::{HashMap, hash_map::Entry};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::pg::{Database, DatabaseName, DbNode};
use super::{DBResult, DatabaseError};

const ENV_PG_DATABASE_URL: &'static str = "PG_DATABASE_URL";

#[derive(Debug, Clone)]
pub struct DbService {
    databases: Arc<RwLock<HashMap<String, Database>>>,
}

impl DbService {
    pub fn new() -> Self {
        Self {
            databases: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[inline]
    pub async fn init_primary(&self) -> sqlx::Result<()> {
        let node = DbNode::Primary;
        let url = std::env::var(ENV_PG_DATABASE_URL)
            .expect("missing required environment variable: PG_DATABASE_URL");
        let name = DatabaseName::Primary;

        self.init_pool(node.pool_options(), url, name.into(), node)
            .await
    }

    #[tracing::instrument(
        name = "db.init_pool",
        skip_all,
        fields(
            db.system = "postgresql",
            db.type = node_type.to_string(),
            db.name = name,
            db.pool.min_connections = options.get_min_connections(),
            db.pool.max_connections = options.get_max_connections(),
            db.host = tracing::field::Empty
        )
    )]
    pub async fn init_pool(
        &self,
        options: PgPoolOptions,
        url: String,
        name: String,
        node_type: DbNode,
    ) -> sqlx::Result<()> {
        let host = url.split('@').nth(1).unwrap_or("unknown-host");

        tracing::Span::current().record("db.host", &host);
        tracing::debug!("initializing PostgreSQL connection pool");

        let pool = match options.connect(&url).await {
            Ok(pool) => pool,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    "failed to establish PostgreSQL connection"
                );
                return Err(err);
            }
        };

        let mut map = self.databases.write().await;

        let db = match map.entry(name) {
            Entry::Occupied(value) => value.into_mut(),
            Entry::Vacant(value) => value.insert(Database::new()),
        };
        db.set_pool(node_type, pool);

        Ok(())
    }

    pub async fn primary(&self) -> DBResult<PgPool> {
        let node = DatabaseName::Primary;

        let db = self.databases.read().await;
        let database = db.get(node.into()).ok_or(DatabaseError::PrimaryMissing)?;

        Ok(database.primary()?.clone())
    }

    pub async fn read(&self) -> DBResult<PgPool> {
        let node = DatabaseName::Primary;

        let db = self.databases.read().await;
        let database = db.get(node.into()).ok_or(DatabaseError::PrimaryMissing)?;

        Ok(database.read()?.clone())
    }
}
