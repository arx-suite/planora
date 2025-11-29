use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult};
use arx_gatehouse::db::{dto::user::CreateUser, repos::UserRepo};
use arx_gatehouse::services::{AuthService, DbManager, auth::cookie::build_cookie};

#[post("/signup")]
async fn signup(
    manager: web::Data<DbManager>,
    auth_service: web::Data<AuthService>,
    payload: web::Json<CreateUser>,
) -> Result<impl Responder, ApiError> {
    let user = payload.into_inner();
    let email = user.email.clone();

    tracing::trace!(%email, "signing up");

    let pool = manager.get_planora_pool().await?;

    let user_repo = UserRepo::new(&pool);
    match user_repo.find_by_email(email.clone()).await? {
        Some(_) => {
            tracing::error!(%email, "email is already registered");
            return Ok(
                HttpResponse::Conflict().json(ApiResult::error("email is already registered"))
            );
        }
        _ => {}
    };

    let inserted_user = user_repo.create_user(user).await?;

    tracing::info!(%email, "user created successfuly");

    tracing::trace!(%email, "generate session token");
    let (access_token, refresh_token) = auth_service.jwt_generate_token(inserted_user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!(%email, "signed up successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::ok("signed up successfully")))
}
