pub mod auth;
pub mod oauth;

use actix_web::Scope;

pub(crate) use super::repo::UserRepo;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(auth::signup)
        .service(auth::verify_email)
        .service(oauth::oauth_start)
}
