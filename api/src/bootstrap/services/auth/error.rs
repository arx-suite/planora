use jsonwebtoken::errors::Error as JwtError;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Jwt error {0}")]
    JwtError(#[from] JwtError),
}
