use actix_multipart::form::MultipartForm;
use actix_web::{HttpRequest, Responder, get, patch, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::user::{UpdateProfileForm, UserProfile, UserRepo};
use arx_gatehouse::services::{AvatarStorage, DbService, S3Service};

#[get("/profile")]
#[tracing::instrument(
    name = "auth.profile.get",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
async fn profile(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::Span::current().record("user_id", &user_id.to_string());

    let pool = db_service.read().await?;
    tracing::debug!("database pool acquired");

    let user = match pool.user_find_by_id(user_id).await? {
        Some(user) => {
            tracing::debug!("user profile loaded");
            UserProfile::from(user)
        }
        None => {
            tracing::warn!(%user_id, "profile not found for user");
            return ApiResult::to_not_found("user not found");
        }
    };

    tracing::info!("profile data returned");

    ApiResult::to_ok_response("profile data", user)
}

#[patch("/profile")]
#[tracing::instrument(
    name = "auth.profile.update",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
async fn update_profile(
    s3: web::Data<S3Service>,
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UpdateProfileForm>,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::Span::current().record("user_id", &user_id.to_string());

    if let Some(avatar) = form.avatar {
        tracing::debug!(
            file_name = ?avatar.file_name,
            content_type = ?avatar.content_type,
            size_bytes = avatar.size,
            "uploading new avatar"
        );

        s3.upload_avatar(user_id, avatar).await?;

        tracing::info!("avatar updated successfully");
    } else {
        tracing::debug!("no avatar update provided");
    }

    tracing::info!("profile update completed");

    ApiResult::to_ok_response("profile updated successfully", ())
}
