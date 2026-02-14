pub mod auth;
pub mod oauth;
pub mod profile;

use actix_web::Scope;

pub(crate) use super::model;
pub(crate) use super::repo::UserRepo;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(auth::signin)
        .service(auth::signout)
        .service(auth::signup)
        .service(auth::verify_email)
        .service(oauth::oauth_start)
        .service(oauth::oauth_callback)
}

pub fn profile_scope() -> Scope {
    Scope::new("/profile").service(profile::get_profile)
}
