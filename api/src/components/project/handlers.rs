use actix_web::Scope;

pub(crate) use super::model;
pub(crate) use super::repo::ProjectRepo;

pub mod project;

pub fn project_scope() -> Scope {
    Scope::new("/project")
        .service(project::project_create)
        .service(project::project_get)
}
