use actix_web::{HttpRequest, Responder, put, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::space::{SpaceInfo, SpaceRepo, UpdateSpace};
use arx_gatehouse::services::DbService;

#[put("")]
async fn update_space(
    db_service: web::Data<DbService>,
    payload: web::Json<UpdateSpace>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let update_space = payload.into_inner();
    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%update_space.space_id, %org_id, "update space information");

    let space_repo = SpaceRepo::new(&pool);
    let updated_space = space_repo.update_space(update_space, org_id).await?;

    tracing::info!(%updated_space.space_id, %org_id, "space updated successfully");
    ApiResult::to_ok_response(
        "space details has been updated successfully",
        SpaceInfo::from(updated_space),
    )
}
