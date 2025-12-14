use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult};
use arx_gatehouse::modules::user::{SigninPayload, UserRepo};
use arx_gatehouse::services::{AuthService, DbManager, auth::cookie::build_cookie};

#[post("/signin")]
async fn signin(
    manager: web::Data<DbManager>,
    auth_service: web::Data<AuthService>,
    payload: web::Json<SigninPayload>,
) -> Result<impl Responder, ApiError> {
    let email = payload.email.to_owned();
    let password = payload.password.to_owned();

    tracing::trace!(%email, "signing in");

    let pool = manager.get_planora_pool().await?;
    let user_repo = UserRepo::new(&pool);

    let user = match user_repo.find_by_email(email.clone()).await? {
        Some(user) => {
            tracing::trace!(%email, "user has been found");
            user
        }
        None => {
            tracing::error!(%email, "invalid email");
            return ApiResult::to_not_found("invalid email");
        }
    };
    match user.password {
        Some(pass) if pass == password => {}
        _ => return ApiResult::to_unauthorized("invalid credentials"),
    }

    tracing::trace!(%email, "valid user credentials");

    tracing::trace!(%email, "generate session token");
    let (access_token, refresh_token) = auth_service.jwt_generate_token(user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!(%email, "signed in successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed in successfully")))
}
