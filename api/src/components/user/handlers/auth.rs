use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use arx_gatehouse::App;
use arx_gatehouse::common::{ApiError, ApiResult};

use super::UserRepo;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedUserInfo {
    pub user: CreateUser,
    pub verification_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

fn email_verification_cache_key(email: &str) -> String {
    format!("email_verification:{email}")
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    tag = "Auth",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User created, verification email sent"),
        (status = 400, description = "Invalid signup data"),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "auth.signup",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        name = payload.username,
        email = payload.email
    )
)]
#[post("/signup")]
async fn signup(
    app: web::Data<App>,
    payload: web::Json<CreateUser>,
) -> Result<impl Responder, ApiError> {
    let user = payload.into_inner();
    let email = user.email.clone();

    let pool = app.db().primary().await?;
    tracing::debug!("database pool acquired");

    match pool.user_find_by_email(email.clone()).await? {
        Some(_) => {
            tracing::debug!("email is already registered");
            return Ok(
                HttpResponse::Conflict().json(ApiResult::error("email is already registered"))
            );
        }
        _ => {}
    };

    // TODO: replace this code
    let verification_code = "123456".to_string();

    if app
        .cache()
        .set_with_ttl(
            &email_verification_cache_key(&verification_code),
            &CachedUserInfo {
                user,
                verification_code: verification_code.clone(),
                created_at: chrono::Utc::now(),
            },
            std::time::Duration::from_secs(15),
        )
        .await
        .is_err()
    {
        tracing::error!("failed to cache the user information");
        return ApiResult::to_internal_error("internal error");
    } else {
        tracing::debug!(db = "redis", "cached the user information");
    };

    match app
        .mail()
        .send_email_verification_code(&email, &verification_code)
        .await
    {
        Ok(_) => {
            tracing::info!("email verification code sent");
            ApiResult::to_ok_response("email verification code has been sent", ())
        }
        Err(err) => {
            tracing::error!("sending mail failed: {err}");
            ApiResult::to_internal_error("failed to send verification code")
        }
    }
}

#[derive(Debug, Clone, Deserialize, utoipa::ToSchema)]
pub struct VerifyEmail {
    pub email: String,
    pub verification_code: String,
}

#[utoipa::path(
    post,
    path = "/auth/verify-email",
    tag = "Auth",
    request_body = VerifyEmail,
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 400, description = "Invalid or expired verification code"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "auth.verify-email",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        email = payload.email
    )
)]
#[post("/verify-email")]
async fn verify_email(
    app: web::Data<App>,
    payload: web::Json<VerifyEmail>,
) -> Result<impl Responder, ApiError> {
    let cache_key = email_verification_cache_key(&payload.email);

    let create_user = match app.cache().get::<CachedUserInfo>(&cache_key).await {
        Ok(create_user) => create_user,
        Err(err) => {
            tracing::error!("cache failed: {err}");
            return ApiResult::to_internal_error("internal error");
        }
    };

    let cached_user = if let Some(user) = create_user {
        if user.verification_code != payload.verification_code {
            return ApiResult::to_unauthorized("Invalid verification code, Try again");
        }

        tracing::debug!("verified the code");
        user
    } else {
        tracing::debug!("Email verification failed");
        return ApiResult::to_bad_request("Email verification failed, Signup again");
    };

    if let Err(err) = app.cache().delete(cache_key).await {
        tracing::warn!("failed to delete cache: {err}");
    }

    let pool = app.db().primary().await?;
    tracing::debug!("database pool acquired");

    pool.user_create_email(
        cached_user.user.username,
        cached_user.user.email,
        cached_user.user.password,
        // TODO: replace this code
        "custom:tag".to_string(),
        cached_user.created_at,
    )
    .await?;

    ApiResult::to_ok_response("Signed up successfully", ())
}
