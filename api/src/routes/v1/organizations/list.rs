use actix_web::{HttpRequest, Responder, get, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::organization::{OrgProfile, OrgRepo};
use arx_gatehouse::services::DbService;

#[get("")]
async fn list_organizations(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;

    tracing::trace!(%user_id, "list organization for the user");

    let pool = db_service.read().await?;

    let orgs = pool.org_find_by_owner_id(user_id).await?;

    let orgs = orgs
        .into_iter()
        .map(|org| org.into())
        .collect::<Vec<OrgProfile>>();

    tracing::info!(%user_id, len = %orgs.len(), "listed organization");

    ApiResult::to_ok_response("organization list", orgs)
}
