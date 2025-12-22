use actix_web::{HttpRequest, Responder, get, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::space::{SpaceInfo, SpaceRepo};
use arx_gatehouse::services::DbService;

#[get("")]
async fn get_spaces_for_org(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, "Listing spaces for organization");

    let space_repo = SpaceRepo::new(&pool);
    let spaces = space_repo.find_by_org_id(org_id).await?;

    let spaces = spaces
        .into_iter()
        .map(|space| space.into())
        .collect::<Vec<SpaceInfo>>();

    tracing::info!(%org_id, len = spaces.len(), "Spaces listed successfully");

    if spaces.len() == 0 {
        return ApiResult::to_no_content("No spaces");
    } else {
        return ApiResult::to_ok_response("spaces", spaces);
    }
}

#[get("{space_id}")]
async fn get_space(
    db_service: web::Data<DbService>,
    path: web::Path<uuid::Uuid>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let space_id = path.into_inner();
    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, %space_id, "find space by id");

    let space_repo = SpaceRepo::new(&pool);
    let space = space_repo.find_by_space_id(space_id, org_id).await?;

    match space {
        Some(space) => {
            tracing::info!(%org_id, %space_id, "found the space successfully");
            return ApiResult::to_ok_response(
                "space has been created successfully",
                SpaceInfo::from(space),
            );
        }
        None => {
            tracing::error!(%org_id, %space_id, "no space were found");
            return ApiResult::to_not_found("space is not found");
        }
    }
}
