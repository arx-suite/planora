#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database operation failed")]
    Sqlx(#[from] sqlx::Error),

    #[error("query building failed")]
    SeaQuery(#[from] sea_query::error::Error),
}

pub type DBResult<T> = Result<T, DatabaseError>;
