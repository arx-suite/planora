use actix_web::{HttpResponse, ResponseError};

use super::ApiResult;
use crate::services::auth::AuthError;
use crate::services::db::DatabaseError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Invalid header value: {0}")]
    ToStrError(#[from] actix_web::http::header::ToStrError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ApiError {
    pub fn bad_request<M: Into<String>>(msg: M) -> Self {
        Self::BadRequest(msg.into())
    }
    pub fn unauthorized<M: Into<String>>(msg: M) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn forbidden<M: Into<String>>(msg: M) -> Self {
        Self::Forbidden(msg.into())
    }

    pub fn not_found<M: Into<String>>(msg: M) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn internal<M: Into<String>>(msg: M) -> Self {
        Self::Internal(msg.into())
    }

    pub fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            ApiError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthError(_) => StatusCode::UNAUTHORIZED,
            ApiError::ToStrError(_) => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn log(&self) {
        match self {
            ApiError::DatabaseError(err) => {
                tracing::error!(target: "api_error", %err, "Database error")
            }
            ApiError::AuthError(err) => {
                tracing::warn!(target: "api_error", %err, "Authentication failed")
            }
            ApiError::ToStrError(err) => {
                tracing::error!(target: "api_error", %err, "Header parsing failed")
            }
            ApiError::Internal(msg) => tracing::error!(target: "api_error", msg, "Internal error"),
            ApiError::BadRequest(msg)
            | ApiError::Unauthorized(msg)
            | ApiError::Forbidden(msg)
            | ApiError::NotFound(msg) => tracing::debug!(target: "api_error", msg, "Client error"),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        self.log();
        let status = self.status_code();

        HttpResponse::build(status).json(ApiResult::<()>::error(self.to_string()))
    }
}
