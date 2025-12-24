use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult};
use arx_gatehouse::modules::user::{CreateUser, UserRepo};
use arx_gatehouse::services::{AuthService, DbService, auth::cookie::build_cookie};

#[post("/signup")]
#[tracing::instrument(
    name = "auth.signup",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        name = payload.username,
        email = payload.email,
        user_id = tracing::field::Empty
    )
)]
async fn signup(
    db_service: web::Data<DbService>,
    auth_service: web::Data<AuthService>,
    payload: web::Json<CreateUser>,
) -> Result<impl Responder, ApiError> {
    let user = payload.into_inner();
    let email = user.email.clone();

    let pool = db_service.primary().await?;
    tracing::debug!("database pool acquired");

    match pool.user_find_by_email(email.clone()).await? {
        Some(_) => {
            tracing::warn!("email is already registered");
            return Ok(
                HttpResponse::Conflict().json(ApiResult::error("email is already registered"))
            );
        }
        _ => {}
    };

    tracing::debug!("inserting user to the database");
    let inserted_user = pool.user_create(user).await?;

    tracing::Span::current().record("user_id", &inserted_user.user_id.to_string());
    tracing::info!("user created successfuly");

    tracing::trace!("generating session tokens");
    let (access_token, refresh_token) = auth_service.jwt_generate_token(inserted_user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!("signed up successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed up successfully")))
}
