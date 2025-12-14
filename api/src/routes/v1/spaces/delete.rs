use actix_web::{HttpRequest, Responder, delete, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::space::{DeleteSpace, SpaceRepo};
use arx_gatehouse::services::DbManager;

#[delete("")]
async fn delete_space(
    manager: web::Data<DbManager>,
    payload: web::Json<DeleteSpace>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let delete_space = payload.into_inner();
    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%delete_space.space_id, %org_id, "delete space for organization");

    let space_repo = SpaceRepo::new(&pool);
    let affected_rows = space_repo
        .delete_by_space_id(delete_space.clone(), org_id)
        .await?;

    if affected_rows == 0 {
        tracing::warn!(%delete_space.space_id, %org_id, "No space found to delete");
        return ApiResult::to_not_found("Space not found");
    }

    tracing::info!(%delete_space.space_id, %org_id, %affected_rows, "Space deleted successfully");

    ApiResult::to_ok_response("Space has been deleted successfully", affected_rows)
}
