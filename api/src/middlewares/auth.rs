use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use crate::components::user::repo::UserRepo;
use arx_gatehouse::App;
use arx_gatehouse::common::ApiError;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: uuid::Uuid,
}

pub struct AuthMiddleware {
    public_paths: Rc<Vec<String>>,
}

impl AuthMiddleware {
    pub fn new(public_paths: Vec<&str>) -> Self {
        Self {
            public_paths: Rc::new(public_paths.into_iter().map(String::from).collect()),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
            public_paths: Rc::clone(&self.public_paths),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    public_paths: Rc<Vec<String>>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path().to_owned();
        let service = Rc::clone(&self.service);
        let public_paths = Rc::clone(&self.public_paths);

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
            tracing::trace!(%path, "incoming request path");

            if public_paths.iter().any(|p| p == &path) {
                return service.call(req).await;
            }

            let token = app.auth().extract_access_token(&req)?;

            let user_id = app
                .auth()
                .jwt_verify_access_token(&token)
                .map_err(ApiError::from)?;

            let pool = app.db().read().await.map_err(ApiError::DatabaseError)?;

            let user = pool
                .user_find_by_id(user_id)
                .await
                .map_err(|_| ApiError::Unauthorized("unauthorized".into()))?
                .ok_or_else(|| ApiError::Unauthorized("unauthorized".into()))?;

            req.extensions_mut().insert(AuthUser { id: user.user_id });

            tracing::trace!(user_id = %user.user_id, "authenticated user attached");

            return service.call(req).await;
        })
    }
}
