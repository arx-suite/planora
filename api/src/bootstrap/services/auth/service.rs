use actix_web::dev::ServiceRequest;

use super::constants::{JWT_ACCESS_TOKEN_KEY, JWT_REFRESH_TOKEN_KEY};
use super::{
    AuthResult,
    constants::{JWT_TOKEN_TYPE_ACCESS, JWT_TOKEN_TYPE_REFRESH},
    jwt::JwtService,
};
use crate::common::ApiError;
use crate::common::utils;

const ENV_JWT_SECRET: &str = "JWT_SECRET";
const ENV_JWT_ACCESS_EXPIRY_MINUTES: &str = "JWT_ACCESS_EXPIRY_MINUTES";
const ENV_JWT_REFRESH_EXPIRY_DAYS: &str = "JWT_REFRESH_EXPIRY_DAYS";

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_service: JwtService,
}

impl AuthService {
    fn from_env() -> Self {
        let secret = utils::get_env::<String>(ENV_JWT_SECRET).unwrap();
        let access_expiry_minutes = utils::get_env::<i64>(ENV_JWT_ACCESS_EXPIRY_MINUTES).unwrap();
        let refresh_expiry_days = utils::get_env::<i64>(ENV_JWT_REFRESH_EXPIRY_DAYS).unwrap();

        Self {
            jwt_service: JwtService::new(secret, access_expiry_minutes, refresh_expiry_days),
        }
    }

    #[inline]
    pub fn jwt_generate_token(&self, user_id: uuid::Uuid) -> AuthResult<(String, String)> {
        self.jwt_service.generate_token(user_id)
    }

    #[inline]
    pub fn jwt_generate_access_token(&self, refresh_token: String) -> AuthResult<String> {
        self.jwt_service.generate_access_token(refresh_token)
    }

    #[inline]
    pub fn jwt_verify_access_token(&self, token: &str) -> AuthResult<uuid::Uuid> {
        let claims = self
            .jwt_service
            .verify_token(JWT_TOKEN_TYPE_ACCESS, token)?;
        Ok(claims.sub)
    }

    #[inline]
    pub fn jwt_verify_refresh_token(&self, token: &str) -> AuthResult<uuid::Uuid> {
        let claims = self
            .jwt_service
            .verify_token(JWT_TOKEN_TYPE_REFRESH, token)?;
        Ok(claims.sub)
    }

    #[inline]
    pub fn extract_access_token(&self, req: &ServiceRequest) -> Result<String, ApiError> {
        let cookie = req
            .cookie(JWT_ACCESS_TOKEN_KEY)
            .ok_or_else(|| ApiError::Unauthorized("Unauthorized".to_string()))?;

        Ok(cookie.value().to_owned())
    }

    #[inline]
    pub fn extract_refresh_token(&self, req: &ServiceRequest) -> Result<String, ApiError> {
        let cookie = req
            .cookie(JWT_REFRESH_TOKEN_KEY)
            .ok_or_else(|| ApiError::Unauthorized("Unauthorized".to_string()))?;

        Ok(cookie.value().to_owned())
    }
}

#[tracing::instrument(
    name = "service.auth",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub fn init() -> AuthService {
    AuthService::from_env()
}
