use actix_multipart::form::MultipartForm;
use actix_web::{HttpRequest, Responder, get, patch, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::user::{UpdateProfileForm, UserProfile, UserRepo};
use arx_gatehouse::services::{AvatarStorage, DbService, S3Service};

#[get("/profile")]
async fn profile(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::trace!(%user_id, "request for profile data");

    let pool = db_service.read().await?;

    let user: UserProfile = match pool.user_find_by_id(user_id).await? {
        Some(user) => {
            tracing::trace!(%user_id, "user has been found");
            user.into()
        }
        None => {
            tracing::error!(%user_id, "failed to retrieve user");
            return ApiResult::to_not_found("user is not found");
        }
    };
    tracing::info!(%user_id, "sending profile data");

    ApiResult::to_ok_response("profile data", user)
}

#[patch("/profile")]
async fn update_profile(
    s3: web::Data<S3Service>,
    req: HttpRequest,
    MultipartForm(form): MultipartForm<UpdateProfileForm>,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::trace!(%user_id, "update the profile");

    if let Some(avatar) = form.avatar {
        tracing::trace!(name = ?avatar.file_name, mime = ?avatar.content_type, size = %avatar.size, "update avatar image");

        s3.upload_avatar(user_id, avatar).await?;
    }

    ApiResult::to_ok_response("Profile updated successfully.", ())
}
