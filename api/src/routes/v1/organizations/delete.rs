use actix_web::{HttpRequest, Responder, delete, web};

use arx_gatehouse::common::{
    ApiError, ApiResult,
    headers::{extract_org_id, extract_user_id},
};
use arx_gatehouse::modules::organization::OrgRepo;
use arx_gatehouse::services::DbService;

#[delete("")]
#[tracing::instrument(
    name = "org.remove",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id,
        org_id
    )
)]
async fn delete_organization(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    let org_id = extract_org_id(&req)?;

    let span = tracing::Span::current();
    span.record("user_id", &tracing::field::display(&user_id));
    span.record("org_id", &tracing::field::display(&org_id));

    let pool = db_service.primary().await?;
    tracing::debug!("database pool acquired");

    let org = match pool.org_find_by_id(org_id).await? {
        Some(org) => org,
        None => {
            tracing::warn!(reason = "org_not_found", "organization not found");
            return ApiResult::to_not_found("organization not found");
        }
    };

    if org.owner_id != user_id {
        tracing::warn!(
            reason = "not_owner",
            "user is not the owner of the organization"
        );
        // NOTE: intentionally response with `NotFound` status code
        return ApiResult::to_not_found("organization not found");
    }

    tracing::debug!("executing organization deletion");
    pool.org_delete_by_id(org_id).await?;

    tracing::info!("organization deleted successfully");

    ApiResult::to_ok_response("organization has been deleted", ())
}
