use jsonwebtoken::errors::Error as JwtError;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("jwt failed {0}")]
    JwtError(#[from] JwtError),

    #[error("invalid token type {0}")]
    InvalidTokenType(String),
}
