use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

use arx_gatehouse::App;
use arx_gatehouse::common::{ApiError, ApiResult, cookie::extract_refresh_token};
use arx_gatehouse::services::auth::cookie::build_cookie_cn;

#[get("/refresh")]
#[tracing::instrument(
    name = "auth.session.refresh",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
async fn refresh(app: web::Data<App>, req: HttpRequest) -> Result<impl Responder, ApiError> {
    let refresh_token = extract_refresh_token(&req)?;
    tracing::trace!("refresh token extracted");

    let access_token = app.auth().jwt_generate_access_token(refresh_token)?;

    let user_id = app.auth().jwt_verify_access_token(&access_token)?;

    tracing::Span::current().record("user_id", &user_id.to_string());

    tracing::debug!("access token successfully refreshed");

    let access_token_cookie = build_cookie_cn(true, access_token);

    tracing::info!("session refreshed");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .json(ApiResult::ok("access token refreshed")))
}
