use actix_web::cookie::{Cookie, SameSite, time};

use super::constants::{JWT_ACCESS_TOKEN_KEY, JWT_REFRESH_TOKEN_KEY};

// TODO: remove these hardcoded values
const DOMAIN: &str = ".planora.sbs";
const SAME_SITE: SameSite = SameSite::Lax;

pub fn build_cookie(
    access_token: String,
    refresh_token: String,
) -> (Cookie<'static>, Cookie<'static>) {
    (
        build_cookie_cn(true, access_token),
        build_cookie_cn(false, refresh_token),
    )
}

pub fn build_cookie_cn(is_access_token: bool, token: String) -> Cookie<'static> {
    let key = if is_access_token {
        JWT_ACCESS_TOKEN_KEY
    } else {
        JWT_REFRESH_TOKEN_KEY
    };

    let cookie = Cookie::build(key, token)
        .domain(DOMAIN)
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SAME_SITE)
        .finish();

    return cookie;
}

pub fn expire_cookie() -> (Cookie<'static>, Cookie<'static>) {
    // Expire both access and refresh token
    (
        expire_cookie_cn(JWT_ACCESS_TOKEN_KEY),
        expire_cookie_cn(JWT_REFRESH_TOKEN_KEY),
    )
}

#[inline]
fn expire_cookie_cn(key: &'static str) -> Cookie<'static> {
    let expire_cookie = Cookie::build(key, "")
        .path("/")
        .secure(false)
        .http_only(true)
        .max_age(time::Duration::seconds(-1))
        .finish();

    return expire_cookie;
}
