use actix_web::Scope;

pub(crate) use super::repo::WorkspaceRepo;

pub mod organization;

pub fn organization_tenant_scope() -> Scope {
    Scope::new("/organization")
}

// this should not be wrapped with tenant middleware
pub fn organization_public_scope() -> Scope {
    Scope::new("/organization").service(organization::organization_create)
}
