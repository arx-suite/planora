use actix_web::{HttpRequest, Responder, delete, web};

use arx_gatehouse::common::{
    ApiError, ApiResult,
    headers::{extract_org_id, extract_user_id},
};
use arx_gatehouse::modules::organization::OrgRepo;
use arx_gatehouse::services::DbService;

#[delete("")]
async fn delete_organization(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    let org_id = extract_org_id(&req)?;

    tracing::trace!(%user_id, %org_id, "delete organization");

    let pool = db_service.primary().await?;
    let org_repo = OrgRepo::new(&pool);

    match org_repo.find_by_orgid(org_id).await? {
        Some(org) if org.owner_id == user_id => {
            tracing::trace!(%user_id, %org_id, "organization has been found");
        }
        _ => {
            tracing::error!(%user_id, %org_id, "failed to retrieve organization for the user");
            return ApiResult::to_not_found("organization is not found for the user");
        }
    };

    let affected_row = org_repo.delete_by_orgid(org_id).await?;

    tracing::info!(%user_id, %org_id, %affected_row, "organization deleted");

    ApiResult::to_ok_response("organization has been deleted", affected_row)
}
