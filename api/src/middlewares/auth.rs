use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use arx_gatehouse::App;
use arx_gatehouse::common::ApiError;
use arx_gatehouse::components::user::model::UserStatus;

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

            let claims = app.auth().authenticate_request(&req)?;

            let pool = app.db().read().await.map_err(ApiError::DatabaseError)?;

            // session check
            let session = pool
                .session_find_by_id(claims.sid)
                .await
                .map_err(ApiError::from)?
                .ok_or_else(|| ApiError::unauthorized("Unauthorized"))?;

            if !session.status.is_active() || chrono::Utc::now() > session.access_expires_at {
                return Err(ApiError::unauthorized("Unauthorized"))?;
            }

            // TODO: last_ip and current_ip check
            // TODO: last_activity checks

            let user = pool
                .user_find_by_id(claims.sub)
                .await
                .map_err(|_| ApiError::Unauthorized("Unauthorized".into()))?
                .ok_or_else(|| ApiError::Unauthorized("Unauthorized".into()))?;

            tracing::trace!(%user.user_id, ?user.status, %user.usertag, ?user.email, "user information");

            // user status check
            match user.status {
                UserStatus::Banned => {
                    return Err(ApiError::forbidden(
                        "Your account has been permanently banned",
                    ))?;
                }
                UserStatus::Deactivated if user.deactivated_at.is_some() => {
                    Err(ApiError::forbidden("Your account has been deactivated"))?
                }
                UserStatus::Suspended => {
                    if let Some(locked_until) = user.locked_until {
                        if locked_until < chrono::Utc::now() {
                            return Err(ApiError::Forbidden(
                                "Account is temporarily locked.".into(),
                            ))?;
                        }
                    }
                }
                _ => {}
            }

            // adding extension
            tracing::trace!(user_id = %user.user_id, "authenticated user attached");
            app.auth().add_user_extension(&req, user);

            return service.call(req).await;
        })
    }
}
