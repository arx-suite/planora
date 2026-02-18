use actix_web::HttpRequest;

use crate::common::ApiError;

const TENANT_HEADER: &str = "X-Organization-Slug";

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub organization_id: uuid::Uuid,
    pub slug: String,
}

impl TenantContext {
    pub fn new(organization_id: uuid::Uuid, slug: String) -> Self {
        Self {
            organization_id,
            slug,
        }
    }

    pub fn insert(self, req: &actix_web::dev::ServiceRequest) {
        <actix_web::dev::ServiceRequest as actix_web::HttpMessage>::extensions_mut(req)
            .insert(self);
    }

    pub fn extract(req: &HttpRequest) -> Result<Self, ApiError> {
        <actix_web::HttpRequest as actix_web::HttpMessage>::extensions(req)
            .get::<TenantContext>()
            .cloned()
            .ok_or_else(|| ApiError::Unauthorized("Tenant context missing".into()))
    }
}

pub fn resolve_org_slug(req: &HttpRequest) -> Option<String> {
    // subdomain
    if let Some(host) = req.connection_info().host().split(':').next() {
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() > 2 {
            return Some(parts[0].to_string());
        }
    }

    // Fallback for mobile / API clients
    req.headers()
        .get(TENANT_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}
