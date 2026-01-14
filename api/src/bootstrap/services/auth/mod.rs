pub mod constants;
pub mod cookie;
mod error;
mod jwt;
pub mod service;

type AuthResult<T> = Result<T, error::AuthError>;

pub use error::AuthError;
pub use service::AuthService;
