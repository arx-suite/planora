use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult};
use arx_gatehouse::modules::user::{SigninPayload, UserRepo};
use arx_gatehouse::services::{AuthService, DbService, auth::cookie::build_cookie};

#[post("/signin")]
#[tracing::instrument(
    name = "auth.signin",
    level = tracing::Level::INFO,
    skip(db_service, auth_service, payload),
    fields(
        email = %payload.email
    )
)]
async fn signin(
    db_service: web::Data<DbService>,
    auth_service: web::Data<AuthService>,
    payload: web::Json<SigninPayload>,
) -> Result<impl Responder, ApiError> {
    let email = payload.email.to_owned();
    let password = payload.password.to_owned();

    let pool = db_service.read().await?;
    tracing::debug!("database pool acquired");

    let span = tracing::info_span!("user.find");
    let _enter = span.enter();
    let user_repo = UserRepo::new(&pool);

    let user = match user_repo.find_by_email(email.clone()).await? {
        Some(user) => {
            tracing::debug!(
                user_id = %user.user_id,
                "user found"
            );
            user
        }
        None => {
            tracing::warn!("signin failed: email not found");
            return ApiResult::to_not_found("invalid email");
        }
    };
    match user.password {
        Some(pass) if pass == password => {
            tracing::debug!(
                user_id = %user.user_id,
                "password validated"
            );
        }
        _ => {
            tracing::warn!(
                user_id = %user.user_id,
                "signin failed: invalid credentials"
            );
            return ApiResult::to_unauthorized("invalid credentials");
        }
    }

    let span = tracing::info_span!("user.jwt_token", user_id = %user.user_id);
    let _enter = span.enter();
    let (access_token, refresh_token) = auth_service.jwt_generate_token(user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!(
        user_id = %user.user_id,
        "user signed in successfully"
    );

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed in successfully")))
}
