use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use arx_gatehouse::App;
use arx_gatehouse::common::ApiError;

use crate::components::user::repo::UserRepo;

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

            let user_id = app.auth().authenticate_request(&req)?;
            let pool = app.db().read().await.map_err(ApiError::DatabaseError)?;

            let user = pool
                .user_find_by_id(user_id)
                .await
                .map_err(|_| ApiError::Unauthorized("Unauthorized".into()))?
                .ok_or_else(|| ApiError::Unauthorized("Unauthorized".into()))?;

            tracing::trace!(%user.user_id, ?user.status, %user.usertag, ?user.email, "user information");

            // user status check
            if user.status.is_banned() {
                return Err(ApiError::Forbidden(
                    "Your account has been permanently banned.".into(),
                ))?;
            }

            if user.status.is_deactivated() || user.deactivated_at.is_some() {
                return Err(ApiError::Forbidden(
                    "Your account has been deactivated.".into(),
                ))?;
            }

            if user.status.is_suspended() {
                if let Some(locked_until) = user.locked_until {
                    if locked_until < chrono::Utc::now() {
                        return Err(ApiError::Forbidden("Account is temporarily locked.".into()))?;
                    }
                }
            }

            // adding extension
            tracing::trace!(user_id = %user.user_id, "authenticated user attached");
            user.add_extension(&req);

            return service.call(req).await;
        })
    }
}
