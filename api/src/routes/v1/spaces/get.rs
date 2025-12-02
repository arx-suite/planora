use actix_web::{HttpRequest, Responder, get, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::db::repos::SpaceRepo;
use arx_gatehouse::services::DbManager;

#[get("")]
async fn get_spaces_for_org(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, "Listing spaces for organization");

    let space_repo = SpaceRepo::new(&pool);
    let spaces = space_repo.find_by_org_id(org_id).await?;

    tracing::info!(%org_id, len = spaces.len(), "Spaces listed successfully");

    if spaces.len() == 0 {
        return ApiResult::to_no_content("No spaces");
    } else {
        return ApiResult::to_ok_response("spaces", spaces);
    }
}

#[get("{space_id}")]
async fn get_space(
    manager: web::Data<DbManager>,
    path: web::Path<uuid::Uuid>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let space_id = path.into_inner();
    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, %space_id, "find space by id");

    let space_repo = SpaceRepo::new(&pool);
    let space = space_repo.find_by_space_id(space_id, org_id).await?;

    match space {
        Some(space) => {
            tracing::info!(%org_id, %space_id, "found the space successfully");
            return ApiResult::to_ok_response("space has been created successfully", space);
        }
        None => {
            tracing::error!(%org_id, %space_id, "no space were found");
            return ApiResult::to_not_found("space is not found");
        }
    }
}
