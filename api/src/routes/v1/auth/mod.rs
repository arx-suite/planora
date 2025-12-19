use actix_web::Scope;

mod profile;
mod refresh;
mod signin;
mod signout;
mod signup;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(signup::signup)
        .service(signin::signin)
        .service(signout::signout)
        .service(refresh::refresh)
        .service(profile::profile)
        .service(profile::update_profile)
}
