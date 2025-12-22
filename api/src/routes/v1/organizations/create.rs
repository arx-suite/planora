use actix_web::{HttpRequest, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::organization::{CreateOrg, OrgProfile, OrgRepo};
use arx_gatehouse::services::DbService;

#[post("")]
async fn create_organization(
    db_service: web::Data<DbService>,
    payload: web::Json<CreateOrg>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let org = payload.into_inner();
    let user_id = extract_user_id(&req)?;

    tracing::trace!(%user_id, "create organization");

    let pool = db_service.primary().await?;

    let inserted_org: OrgProfile = pool.org_create(&org, user_id).await?.into();

    tracing::info!(%user_id, "created organization");

    ApiResult::to_ok_response("organization has been created", inserted_org)
}
