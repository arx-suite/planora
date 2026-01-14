#[derive(Debug, thiserror::Error)]
pub enum MailError {
    #[error("Failed to build the email: {0}")]
    BuildError(#[from] lettre::error::Error),

    #[error("Failed to send the email: {0}")]
    SendError(#[from] lettre::transport::smtp::Error),

    #[error("Invalid address: {0}")]
    InvalidAddress(#[from] lettre::address::AddressError),
}

pub type MailResult<T> = std::result::Result<T, MailError>;
