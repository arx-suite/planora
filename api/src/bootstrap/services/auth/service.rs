use actix_web::cookie::{Cookie, SameSite, time};
use chrono::{Duration, Utc};
use jsonwebtoken::*;
use uuid::Uuid;

use crate::common::ApiError;
use crate::common::utils;

use super::AuthError;
use super::types::*;

const ENV_JWT_SECRET: &str = "JWT_SECRET";
const ENV_JWT_ACCESS_EXPIRY_MINUTES: &str = "JWT_ACCESS_EXPIRY_MINUTES";
const ENV_JWT_REFRESH_EXPIRY_DAYS: &str = "JWT_REFRESH_EXPIRY_DAYS";
const ACCESS_COOKIE: &str = "access_token";
const REFRESH_COOKIE: &str = "refresh_token";
// TODO: only for temporary use
const DOMAIN: &str = ".planora.sbs";

#[derive(Debug, Clone)]
pub struct AuthService {
    secret: String,
    access_ttl_min: i64,
    refresh_ttl_days: i64,
    cookie: AuthCookieConfig,
}

impl AuthService {
    fn new(
        secret: String,
        access_ttl_min: i64,
        refresh_ttl_days: i64,
        cookie: AuthCookieConfig,
    ) -> Self {
        Self {
            secret,
            access_ttl_min,
            refresh_ttl_days,
            cookie,
        }
    }

    // Auth middleware helper
    pub fn authenticate_request(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<uuid::Uuid, ApiError> {
        let token = self.extract_access_token(req)?;
        let claims = self.verify_access(&token)?;

        Ok(claims.sub)
    }

    // jwt
    pub fn issue_token_pair(&self, user_id: Uuid) -> Result<TokenPair, ApiError> {
        let session_id = Uuid::new_v4();

        Ok(TokenPair {
            access: self.issue_access_token(user_id, session_id)?,
            refresh: self.issue_refresh_token(user_id, session_id)?,
        })
    }

    pub fn issue_access_token(&self, user_id: Uuid, session_id: Uuid) -> Result<String, ApiError> {
        self.encode(self.build_claims(
            user_id,
            session_id,
            TokenType::Access,
            Duration::minutes(self.access_ttl_min),
        ))
    }

    pub fn issue_refresh_token(&self, user_id: Uuid, session_id: Uuid) -> Result<String, ApiError> {
        self.encode(self.build_claims(
            user_id,
            session_id,
            TokenType::Refresh,
            Duration::days(self.refresh_ttl_days),
        ))
    }

    pub fn verify_access(&self, token: &str) -> Result<JwtClaims, ApiError> {
        let claims = self.decode(token)?;
        if claims.typ != TokenType::Access {
            return Err(ApiError::Unauthorized("Invalid access token".into()));
        }
        Ok(claims)
    }

    pub fn verify_refresh(&self, token: &str) -> Result<JwtClaims, ApiError> {
        let claims = self.decode(token)?;
        if claims.typ != TokenType::Refresh {
            return Err(ApiError::Unauthorized("Invalid refresh token".into()));
        }
        Ok(claims)
    }

    pub fn rotate_access_token(&self, refresh_token: &str) -> Result<String, ApiError> {
        let claims = self.verify_refresh(refresh_token)?;
        self.issue_access_token(claims.sub, claims.sid)
    }

    // cookie
    pub fn issue_auth_cookies(&self, user_id: uuid::Uuid) -> Result<AuthCookies, ApiError> {
        let pair = self.issue_token_pair(user_id)?;

        Ok(AuthCookies {
            access: self.build_cookie(ACCESS_COOKIE, pair.access, self.access_ttl_min * 60),
            refresh: self.build_cookie(REFRESH_COOKIE, pair.refresh, self.refresh_ttl_days * 86400),
        })
    }

    pub fn clear_auth_cookies(&self) -> AuthCookies {
        AuthCookies {
            access: self.expire_cookie(ACCESS_COOKIE),
            refresh: self.expire_cookie(REFRESH_COOKIE),
        }
    }

    pub fn extract_access_token(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, ApiError> {
        let cookie = req
            .cookie(ACCESS_COOKIE)
            .ok_or_else(|| ApiError::Unauthorized("Missing access token".into()))?;

        Ok(cookie.value().to_string())
    }

    pub fn extract_refresh_token(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, ApiError> {
        let cookie = req
            .cookie(REFRESH_COOKIE)
            .ok_or_else(|| ApiError::Unauthorized("Missing refresh token".into()))?;

        Ok(cookie.value().to_string())
    }

    #[inline]
    pub fn extract_access_claims(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<JwtClaims, ApiError> {
        self.verify_access(&self.extract_access_token(req)?)
    }

    #[inline]
    pub fn extract_refresh_claims(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<JwtClaims, ApiError> {
        self.verify_refresh(&self.extract_refresh_token(req)?)
    }

    fn build_claims(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        typ: TokenType,
        ttl: Duration,
    ) -> JwtClaims {
        let now = Utc::now();

        JwtClaims {
            sub: user_id,
            jti: Uuid::new_v4(),
            sid: session_id,
            typ,
            iat: now.timestamp() as usize,
            exp: (now + ttl).timestamp() as usize,
        }
    }

    fn encode(&self, claims: JwtClaims) -> Result<String, ApiError> {
        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(AuthError::from)?)
    }

    fn decode(&self, token: &str) -> Result<JwtClaims, ApiError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        Ok(decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|d| d.claims)
        .map_err(AuthError::from)?)
    }

    fn build_cookie(
        &self,
        name: &'static str,
        value: String,
        max_age_secs: i64,
    ) -> Cookie<'static> {
        let mut builder = Cookie::build(name, value)
            .path("/")
            .http_only(true)
            .secure(self.cookie.secure)
            .same_site(self.cookie.same_site)
            .max_age(time::Duration::seconds(max_age_secs));

        if let Some(domain) = &self.cookie.domain {
            builder = builder.domain(domain.clone());
        }

        builder.finish()
    }

    fn expire_cookie(&self, name: &'static str) -> Cookie<'static> {
        let mut builder = Cookie::build(name, "")
            .path("/")
            .http_only(true)
            .secure(self.cookie.secure)
            .same_site(self.cookie.same_site)
            .max_age(time::Duration::seconds(-1));

        if let Some(domain) = &self.cookie.domain {
            builder = builder.domain(domain.clone());
        }

        if self.cookie.secure {
            builder = builder.secure(true);
        }

        builder.finish()
    }
}

#[tracing::instrument(
    name = "service.auth",
    skip_all,
    level = tracing::Level::DEBUG
)]
pub fn init() -> AuthService {
    let secret = utils::get_env::<String>(ENV_JWT_SECRET).unwrap();
    let access_ttl_min = utils::get_env::<i64>(ENV_JWT_ACCESS_EXPIRY_MINUTES).unwrap();
    let refresh_ttl_days = utils::get_env::<i64>(ENV_JWT_REFRESH_EXPIRY_DAYS).unwrap();

    // TODO: these are hardcoded values
    //       get values from the env
    let cookie_config = AuthCookieConfig {
        domain: Some(DOMAIN.into()),
        same_site: SameSite::Lax,
        secure: false,
    };

    AuthService::new(secret, access_ttl_min, refresh_ttl_days, cookie_config)
}
