use actix_web::{Scope, web};

mod auth;
mod health;
mod organizations;
mod projects;
mod spaces;

pub fn v1_scope() -> Scope {
    let scope = web::scope("/v1")
        .service(health::health_check)
        .service(auth::auth_scope())
        .service(organizations::organizations_scope())
        .service(spaces::spaces_scope())
        .service(projects::projects_scope());
    scope
}
