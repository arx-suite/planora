use actix_web::Scope;

pub(crate) use super::repo::WorkspaceRepo;

pub mod organization;

pub fn organization_scope() -> Scope {
    Scope::new("/organization").service(organization::organization_create)
}
