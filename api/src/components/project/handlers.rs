use actix_web::Scope;

pub(crate) use super::model;
pub(crate) use super::repo::{ProjectRepo, TaskRepo};

pub mod project;
pub mod task;

pub fn project_scope() -> Scope {
    Scope::new("/project")
        .service(project::project_create)
        .service(project::project_get)
}

pub fn task_scope() -> Scope {
    Scope::new("/task").service(task::task_create)
}
