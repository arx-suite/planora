use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use super::ApiResult;
use crate::services::auth::AuthError;
use crate::services::db::DatabaseError;
use crate::services::s3::S3Error;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("S3 Error: {0}")]
    S3Error(#[from] S3Error),

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
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let (status, message) = match self {
            ApiError::DatabaseError(err) => {
                use sqlx::Error::*;

                match err {
                    DatabaseError::Sqlx(sqlx_err) => match sqlx_err {
                        Database(db_err) => {
                            if let Some(status) = status_from_sql_error_code(db_err.code()) {
                                tracing::debug!("database sqlx error: {err}");
                                (status, db_err.message())
                            } else {
                                tracing::error!("database sqlx error: {err}");
                                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                            }
                        }

                        err => {
                            tracing::error!("database sqlx error: {err}");
                            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                        }
                    },
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
                }
            }
            ApiError::AuthError(err) => {
                tracing::warn!(target: "api_error", %err, "Authentication failed");
                (StatusCode::UNAUTHORIZED, "Authentication failed")
            }
            ApiError::ToStrError(err) => {
                tracing::debug!(target: "api_error", %err, "To str error");
                (StatusCode::BAD_REQUEST, "bad request")
            }
            ApiError::BadRequest(message) => {
                tracing::debug!("BadRequest Response: {message}");
                (StatusCode::BAD_REQUEST, message.as_str())
            }
            ApiError::Unauthorized(message) => {
                tracing::debug!("Unauthorized Response: {message}");
                (StatusCode::UNAUTHORIZED, message.as_str())
            }
            ApiError::Forbidden(message) => {
                tracing::debug!("Forbidden Response: {message}");
                (StatusCode::FORBIDDEN, message.as_str())
            }
            ApiError::NotFound(message) => {
                tracing::debug!("NotFound Response: {message}");
                (StatusCode::NOT_FOUND, message.as_str())
            }
            ApiError::Internal(err) => {
                tracing::error!(target: "api_error", %err, "S3 Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            ApiError::S3Error(err) => {
                tracing::error!(target: "api_error", %err, "S3 error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };

        HttpResponse::build(status).json(ApiResult::<()>::error(message))
    }
}

#[inline]
fn status_from_sql_error_code(code: Option<std::borrow::Cow<'_, str>>) -> Option<StatusCode> {
    code.as_deref()
        .and_then(|c| c.strip_prefix("A0"))
        .and_then(|s| s.parse::<u16>().ok())
        .and_then(|n| StatusCode::from_u16(n).ok())
}
