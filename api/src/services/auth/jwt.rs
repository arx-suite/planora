use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use super::{
    AuthError, AuthResult,
    constants::{JWT_TOKEN_TYPE_ACCESS, JWT_TOKEN_TYPE_REFRESH},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub token_type: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone)]
pub struct JwtService {
    secret: String,
    access_expiry_minutes: i64,
    refresh_expiry_days: i64,
}

impl JwtService {
    pub(crate) fn new(
        secret: String,
        access_expiry_minutes: i64,
        refresh_expiry_days: i64,
    ) -> Self {
        Self {
            secret,
            access_expiry_minutes,
            refresh_expiry_days,
        }
    }

    pub(crate) fn generate_token(&self, user_id: uuid::Uuid) -> AuthResult<(String, String)> {
        let access_claims = self.new_claims(user_id, true);
        let refresh_claims = self.new_claims(user_id, false);

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        tracing::trace!(%user_id, "generated jwt token");
        Ok((access_token, refresh_token))
    }

    pub(crate) fn verify_token(&self, token_type: &str, token: &str) -> AuthResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        let claims = token_data.claims;

        if claims.token_type != token_type {
            return Err(AuthError::InvalidTokenType(claims.token_type.into()));
        }

        tracing::trace!(%claims.sub, %claims.token_type, "verified jwt token");

        Ok(claims)
    }

    pub(crate) fn generate_access_token(&self, refresh_token: String) -> AuthResult<String> {
        let refresh_data = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        let claims = refresh_data.claims;

        if claims.token_type != JWT_TOKEN_TYPE_REFRESH {
            return Err(AuthError::InvalidTokenType(claims.token_type));
        }

        let new_access_claims = self.new_claims(claims.sub, true);
        let new_access_token = encode(
            &Header::default(),
            &new_access_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        tracing::trace!(%claims.sub, "generated jwt access token");

        Ok(new_access_token)
    }

    #[inline]
    pub(crate) fn new_claims(&self, sub: uuid::Uuid, is_access_token: bool) -> Claims {
        let now = Utc::now();

        let (token_type, duration) = if is_access_token {
            (
                JWT_TOKEN_TYPE_ACCESS,
                Duration::minutes(self.access_expiry_minutes),
            )
        } else {
            (
                JWT_TOKEN_TYPE_REFRESH,
                Duration::days(self.refresh_expiry_days),
            )
        };

        tracing::trace!(%token_type, %sub, %now, "jwt claims");

        Claims {
            sub: sub.to_owned(),
            token_type: token_type.into(),
            iat: now.timestamp() as usize,
            exp: (now + duration).timestamp() as usize,
        }
    }
}
