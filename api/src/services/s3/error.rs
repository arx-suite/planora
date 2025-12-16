use aws_sdk_s3::{error::ProvideErrorMetadata, primitives::ByteStreamError};

pub type S3Result<T> = std::result::Result<T, Error>;
pub type S3Error = Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Configuration / Startup
    #[error("S3 configuration error: {0}")]
    Config(String),

    #[error("S3 client initialization failed")]
    ClientInit,

    #[error("failed to connect to S3 endpoint")]
    Connection,

    #[error("request to S3 timed out")]
    Timeout,

    // Auth / Permissions
    #[error("invalid S3 credentials")]
    InvalidCredentials,

    #[error("access denied to S3 resource")]
    AccessDenied,

    // Resource
    #[error("S3 bucket not found")]
    BucketNotFound,

    #[error("S3 object not found")]
    ObjectNotFound,

    #[error("S3 bucket already exists")]
    BucketAlreadyExists,

    #[error("Unsupported content type")]
    UnsupportedContent,

    #[error("Unsupported content type")]
    UnsupportedExtension,

    // Data
    #[error("failed to read object data")]
    ByteStream(ByteStreamError),

    #[error("invalid object data: {0}")]
    InvalidData(String),

    #[error("s3 service error ({code}): {message}")]
    Service { code: String, message: String },

    #[error("s3 transport error: {0}")]
    Transport(String),

    #[error("unexpected s3 error: {0}")]
    Unknown(String),
}

impl<T> From<T> for Error
where
    T: ProvideErrorMetadata,
{
    fn from(err: T) -> Self {
        let code = err.code().unwrap_or("unknown");
        let message = err.message().unwrap_or("no message");

        match code {
            // Auth
            "InvalidAccessKeyId" | "SignatureDoesNotMatch" => Error::InvalidCredentials,

            "AccessDenied" => Error::AccessDenied,
            // Resource
            "NoSuchBucket" => Error::BucketNotFound,
            "NoSuchKey" => Error::ObjectNotFound,
            "BucketAlreadyExists" => Error::BucketAlreadyExists,

            // Network / Redirect
            "PermanentRedirect" | "TemporaryRedirect" => {
                Error::Config("incorrect S3 endpoint or path-style mismatch".into())
            }

            "RequestTimeout" => Error::Timeout,

            // Fallback
            _ => Error::Service {
                code: code.to_string(),
                message: message.to_string(),
            },
        }
    }
}
