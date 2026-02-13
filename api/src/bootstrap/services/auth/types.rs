use actix_web::cookie::{Cookie, SameSite};

// jwt
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaims {
    /// User id
    pub sub: uuid::Uuid,
    /// Session id
    pub sid: uuid::Uuid,
    /// Token type
    pub typ: TokenType,
    /// Issued at
    pub iat: usize,
    /// Expired at
    pub exp: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

// session
#[derive(Debug, Clone)]
pub struct AuthCookieConfig {
    pub domain: Option<String>,
    pub secure: bool,
    pub same_site: SameSite,
}

#[derive(Debug, Clone)]
pub struct AuthCookies {
    pub access: Cookie<'static>,
    pub refresh: Cookie<'static>,
}
