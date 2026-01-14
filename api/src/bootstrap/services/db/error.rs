use super::pg::DbNode;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database operation failed")]
    Sqlx(#[from] sqlx::Error),

    #[error("query building failed")]
    SeaQuery(#[from] sea_query::error::Error),

    #[error("database node `{0}` is not configured")]
    NodeNotConfigured(DbNode),

    #[error("primary database node is required but missing")]
    PrimaryMissing,
}

pub type DBResult<T> = Result<T, DatabaseError>;
