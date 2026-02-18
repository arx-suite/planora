use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

use super::UserRepo;
use crate::App;
use crate::common::{ApiError, ApiResult};

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
        (status = 200, description = "Verification email sent"),
        // TODO: common response
        // (status = 400, description = "Invalid signup data"),
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
        (status = 201, description = "Email verified successfully"),
        (status = 401, description = "Invalid or expired verification code"),
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
    req: HttpRequest,
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

    let user = pool
        .user_create_email(
            cached_user.user.username,
            cached_user.user.email,
            cached_user.user.password,
            // TODO: replace this code
            "custom:tag".to_string(),
            cached_user.created_at,
        )
        .await?;

    let ip = req
        .peer_addr()
        .and_then(|v| Some(sqlx::types::ipnetwork::IpNetwork::from(v.ip())));

    // user-agent header
    let req = actix_web::dev::ServiceRequest::from_request(req);
    let ua = app
        .auth()
        .parse_user_agent(&req)
        .unwrap_or(app.auth().user_agent_default());

    let now = chrono::Utc::now();
    let session = pool
        .session_create(
            user.user_id,
            ua.browser,
            ip,
            None,
            Some(ua.device),
            Some(ua.os),
            None,
            now + chrono::Duration::minutes(app.auth().access_ttl_min),
            now + chrono::Duration::days(app.auth().refresh_ttl_days),
        )
        .await?;

    let cookies = app
        .auth()
        .issue_auth_cookies(user.user_id, session.session_id)?;

    tracing::info!(%user.user_id, %payload.email, "user signed up successfully");

    Ok(HttpResponse::Created()
        .cookie(cookies.access)
        .cookie(cookies.refresh)
        .json(ApiResult::ok("Signed up successfully")))
}

#[derive(Debug, Clone, Deserialize, utoipa::ToSchema)]
pub struct SigninPayload {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/auth/signin",
    tag = "Auth",
    request_body = SigninPayload,
    responses(
        (status = 401, description = "Authentication failed"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "auth.signin",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        email = %payload.email,
        user_id = tracing::field::Empty
    )
)]
#[post("/signin")]
async fn signin(
    app: web::Data<App>,
    payload: web::Json<SigninPayload>,
) -> Result<impl Responder, ApiError> {
    let email = payload.email.to_owned();
    let password = payload.password.to_owned();

    tracing::debug!("database pool acquired");
    let pool = app.db().primary().await?;

    let user = pool
        .user_find_by_email(email)
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Authentication failed".into()))?;

    match user.password_hash {
        Some(p) if p == password => {}
        _ => return Err(ApiError::Unauthorized("Authentication failed".into())),
    }

    if !user.status.is_signin_allowed() {
        return ApiResult::to_forbidden("User cannot login.");
    }

    let now = chrono::Utc::now();
    // TODO: retrieve user device and network information
    let session = pool
        .session_create(
            user.user_id,
            "unknown".into(),
            None,
            None,
            None,
            None,
            None,
            now + chrono::Duration::minutes(app.auth().access_ttl_min),
            now + chrono::Duration::days(app.auth().refresh_ttl_days),
        )
        .await?;

    let cookies = app
        .auth()
        .issue_auth_cookies(user.user_id, session.session_id)?;

    tracing::info!(%user.user_id, %payload.email, "user signed in successfully");

    Ok(HttpResponse::Ok()
        .cookie(cookies.access)
        .cookie(cookies.refresh)
        .json(ApiResult::ok("Signed in successfully")))
}

#[utoipa::path(
    post,
    path = "/auth/signout",
    tag = "Auth",
    responses(
        (status = 200, description = "Signed out successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    name = "auth.signout",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty,
        session_id = tracing::field::Empty
    )
)]
#[post("/signout")]
async fn signout(req: HttpRequest, app: web::Data<App>) -> Result<impl Responder, ApiError> {
    let user = app.auth().extract_user_extension(&req)?;
    let session = app.auth().extract_session_extension(&req)?;

    let span = tracing::Span::current();
    span.record("user_id", &user.user_id.to_string());
    span.record("session_id", &session.session_id.to_string());

    // revoke session
    let pool = app.db().primary().await?;
    pool.session_revoke(session.session_id, "user_signout")
        .await?;

    // add the tokens while expiry
    let cache = app.cache();

    let ttl = (session.refresh_expires_at - chrono::Utc::now())
        .num_seconds()
        .max(0) as u64;

    let key = revoke_key(session.session_id);
    cache
        .set_with_ttl(&key, &key, std::time::Duration::from_secs(ttl))
        .await
        .map_err(|_| ApiError::internal("Internal error"))?;

    let cookies = app.auth().clear_auth_cookies();

    Ok(HttpResponse::Ok()
        .cookie(cookies.access)
        .cookie(cookies.refresh)
        .json(ApiResult::ok("Signed out successfully")))
}

// TODO: add this code inside the AuthService
#[inline]
fn revoke_key(session_id: uuid::Uuid) -> String {
    format!("revoked:session:{}", session_id)
}
