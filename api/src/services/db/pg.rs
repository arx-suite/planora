use sqlx::{PgPool, postgres::PgPoolOptions};
use std::collections::hash_map::{Entry, HashMap};

use super::{DBResult, DatabaseError};

#[derive(Debug, Clone)]
pub struct Database {
    nodes: HashMap<DbNode, PgPool>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn pool(&self, node: DbNode) -> DBResult<&PgPool> {
        self.nodes
            .get(&node)
            .ok_or(DatabaseError::NodeNotConfigured(node))
    }

    /// Register or override a pool for a node
    pub fn set_pool(&mut self, node: DbNode, pool: PgPool) {
        match self.nodes.entry(node) {
            Entry::Occupied(_) => {
                tracing::warn!(%node, "overriding existing database pool");
            }
            Entry::Vacant(_) => {
                tracing::info!(%node, "registering database pool");
            }
        }

        self.nodes.insert(node, pool);
    }

    pub fn primary(&self) -> DBResult<&PgPool> {
        self.pool(DbNode::Primary)
    }

    pub fn read(&self) -> DBResult<&PgPool> {
        match self.nodes.get(&DbNode::ReadReplica) {
            Some(pool) => Ok(pool),
            None => {
                tracing::debug!("read replica not configured, falling back to primary");
                self.primary()
            }
        }
    }

    // pub fn validate(&self) -> DBResult<()> {
    //     if !self.nodes.contains_key(&DbNode::Primary) {
    //         tracing::error!("primary database is missing");
    //         return Err(DatabaseError::PrimaryMissing);
    //     }
    //     Ok(())
    // }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DbNode {
    /// Primary node for handling transactions
    Primary,
    /// Read-only replica
    ReadReplica,
}

impl DbNode {
    pub fn pool_options(self) -> PgPoolOptions {
        match self {
            DbNode::Primary => PgPoolOptions::new().max_connections(15).min_connections(1),
            DbNode::ReadReplica => PgPoolOptions::new().max_connections(10).min_connections(1),
        }
    }
}

impl std::fmt::Display for DbNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Primary => write!(f, "primary"),
            Self::ReadReplica => write!(f, "read_replica"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseName {
    /// primary database
    Primary,
}

impl From<DatabaseName> for String {
    fn from(value: DatabaseName) -> Self {
        use DatabaseName::*;

        match value {
            Primary => "primary".into(),
        }
    }
}

impl From<DatabaseName> for &str {
    fn from(value: DatabaseName) -> Self {
        use DatabaseName::*;

        match value {
            Primary => "primary",
        }
    }
}
