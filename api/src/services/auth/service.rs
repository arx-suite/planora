use std::env;

use super::{
    AuthResult,
    constants::{JWT_TOKEN_TYPE_ACCESS, JWT_TOKEN_TYPE_REFRESH},
    jwt::JwtService,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_service: JwtService,
}

impl AuthService {
    fn from_env() -> Self {
        let secret =
            env::var("JWT_SECRET").expect("missing required environment variable: JWT_SECRET");

        let access_expiry_minutes = env::var("JWT_ACCESS_EXPIRY_MINUTES")
            .expect("missing required environment variable: JWT_ACCESS_EXPIRY_MINUTES")
            .parse()
            .expect("JWT_ACCESS_EXPIRY_MINUTES must be a number");

        let refresh_expiry_days = env::var("JWT_REFRESH_EXPIRY_DAYS")
            .expect("missing required environment variable: JWT_REFRESH_EXPIRY_DAYS")
            .parse()
            .expect("JWT_REFRESH_EXPIRY_DAYS must be a number");

        tracing::info!("Auth service initialized!");

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
}

#[tracing::instrument(name = "service.auth", skip_all)]
pub fn init() -> AuthService {
    AuthService::from_env()
}
