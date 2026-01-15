use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::App;
use arx_gatehouse::common::{ApiError, ApiResult};
use arx_gatehouse::modules::user::{SigninPayload, UserRepo};
use arx_gatehouse::services::auth::cookie::build_cookie;

#[post("/signin")]
#[tracing::instrument(
    name = "auth.signin",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        email = %payload.email,
        user_id = tracing::field::Empty
    )
)]
async fn signin(
    app: web::Data<App>,
    payload: web::Json<SigninPayload>,
) -> Result<impl Responder, ApiError> {
    let email = payload.email.to_owned();
    let password = payload.password.to_owned();

    let pool = app.db().read().await?;
    tracing::debug!("database pool acquired");

    let user = match pool.user_find_by_email(email.clone()).await? {
        Some(user) => {
            tracing::Span::current().record("user_id", &user.user_id.to_string());
            tracing::debug!(user_id = %user.user_id, "user found");
            user
        }
        None => {
            tracing::warn!("signin failed: email unknown email");
            return ApiResult::to_not_found("invalid email");
        }
    };

    match user.password {
        Some(pass) if pass == password => {
            tracing::debug!(user_id = %user.user_id, "credentials validated");
        }
        _ => {
            tracing::warn!(user_id = %user.user_id, "signin failed: invalid credentials");
            return ApiResult::to_unauthorized("invalid credentials");
        }
    }

    let (access_token, refresh_token) = app.auth().jwt_generate_token(user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!(user_id = %user.user_id, "user signed in successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed in successfully")))
}
