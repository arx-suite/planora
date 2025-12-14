mod error;
pub(crate) mod helpers;

pub use error::DatabaseError;

pub type DBResult<T> = Result<T, DatabaseError>;
