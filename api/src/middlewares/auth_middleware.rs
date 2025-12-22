use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    http::header::HeaderValue,
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;

use arx_gatehouse::{
    common::{ApiError, constants::X_USER_ID_HEADER, cookie::extract_access_token},
    modules::user::UserRepo,
    services::{AuthService, DbService},
};

pub struct AuthMiddleware {
    public_paths: Rc<Vec<String>>,
    auth_service: Rc<AuthService>,
    db_service: Rc<DbService>,
}

impl AuthMiddleware {
    pub fn new(public_paths: Vec<&str>, auth_service: AuthService, db_service: DbService) -> Self {
        Self {
            public_paths: Rc::new(public_paths.into_iter().map(String::from).collect()),
            auth_service: Rc::new(auth_service),
            db_service: Rc::new(db_service),
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
            auth_service: Rc::clone(&self.auth_service),
            db_service: Rc::clone(&self.db_service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    public_paths: Rc<Vec<String>>,
    auth_service: Rc<AuthService>,
    db_service: Rc<DbService>,
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

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let public_paths = Rc::clone(&self.public_paths);
        let service = Rc::clone(&self.service);
        let db_service = Rc::clone(&self.db_service);
        let auth_service = Rc::clone(&self.auth_service);

        Box::pin(async move {
            tracing::trace!(%path, "incoming request path");

            if public_paths.iter().any(|p| path.eq(p)) {
                tracing::debug!(%path, "public path — skipping authentication");
                return service.call(req).await;
            }

            tracing::debug!(%path, "checking authentication");

            let pool = db_service
                .read()
                .await
                .map_err(|value| ApiError::DatabaseError(value))?;

            // Extract token cookie
            let token = extract_access_token(&req)?;

            // Verify token
            let user_id = auth_service
                .jwt_verify_access_token(&token)
                .map_err(ApiError::from)?;

            // let user_repo = UserRepo::new(&pool);
            let user = pool
                .user_find_by_id(user_id)
                .await
                .map_err(|_| ApiError::Unauthorized("unauthorized".to_string()))?
                .ok_or_else(|| ApiError::Unauthorized("unauthorized".to_string()))?;

            let user_id_header_val = HeaderValue::from_str(user.user_id.to_string().as_str())
                .map_err(|err| {
                    tracing::error!(%err, "parsing into HeaderValue failed");
                    ApiError::Internal("internal error".to_string())
                })?;

            req.headers_mut()
                .insert(X_USER_ID_HEADER, user_id_header_val);

            tracing::trace!(user_id = %user.user_id, "attached user ID to request headers");

            return service.call(req).await;
        })
    }
}
