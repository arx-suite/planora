use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, web};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use std::rc::Rc;

use arx_gatehouse::App;
use arx_gatehouse::common::ApiError;
use arx_gatehouse::components::workspace::repo::WorkspaceRepo;
use arx_gatehouse::context::tenant::{TenantContext, resolve_org_slug};

pub struct TenantMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TenantMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TenantMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TenantMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct TenantMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TenantMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let app = match req.app_data::<web::Data<App>>().cloned() {
            Some(app) => app,
            None => {
                return Box::pin(async {
                    tracing::debug!("App state missing");
                    Err(ApiError::Internal("Internal error".into()).into())
                });
            }
        };

        Box::pin(async move {
            let slug = if let Some(org) = resolve_org_slug(req.request()) {
                org
            } else {
                return Err(ApiError::not_found("There is no organizations"))?;
            };

            let pool = app.db().read().await.map_err(ApiError::DatabaseError)?;

            let org = pool
                .organization_find_by_subdomain(slug.clone())
                .await
                .map_err(ApiError::from)?
                .ok_or_else(|| ApiError::NotFound("No organization found".into()))?;

            // adding extension
            tracing::trace!(org_id = %org.organization_id, "tenant context attached");
            let tenant = TenantContext::new(org.organization_id, slug);
            tenant.insert(&req);

            return service.call(req).await;
        })
    }
}
