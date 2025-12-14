use actix_web::{HttpRequest, Responder, post, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::db::dto::organization::{CreateOrg, OrgProfile};
use arx_gatehouse::db::repos::OrgRepo;
use arx_gatehouse::services::DbManager;

#[post("")]
async fn create_organization(
    manager: web::Data<DbManager>,
    payload: web::Json<CreateOrg>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let org = payload.into_inner();
    let user_id = extract_user_id(&req)?;

    tracing::trace!(%user_id, "create organization");

    let pool = manager.get_planora_pool().await?;
    let org_repo = OrgRepo::new(&pool);

    let inserted_org: OrgProfile = org_repo.create_org(&org, user_id).await?.into();

    tracing::info!(%user_id, "created organization");

    ApiResult::to_ok_response("organization has been created", inserted_org)
}
