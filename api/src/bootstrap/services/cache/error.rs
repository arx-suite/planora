use deadpool_redis::PoolError;

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Failed to get the connection")]
    PoolError(#[from] PoolError),

    #[error("Redis failed")]
    RedisError(#[from] deadpool_redis::redis::RedisError),

    #[error("Json serde failed")]
    JsonSerdeError(#[from] serde_json::Error),
}

pub type CacheResult<T> = std::result::Result<T, CacheError>;
