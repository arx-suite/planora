use actix_web::{HttpRequest, Responder, get, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::db::dto::organization::OrgProfile;
use arx_gatehouse::db::repos::OrgRepo;
use arx_gatehouse::services::DbManager;

#[get("")]
async fn list_organizations(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;

    tracing::trace!(%user_id, "list organization for the user");

    let pool = manager.get_planora_pool().await?;

    let org_repo = OrgRepo::new(&pool);
    let orgs = org_repo.find_by_ownerid(user_id).await?;

    let orgs = orgs
        .into_iter()
        .map(|org| org.into())
        .collect::<Vec<OrgProfile>>();

    tracing::info!(%user_id, len = %orgs.len(), "listed organization");

    ApiResult::to_ok_response("organization list", orgs)
}
