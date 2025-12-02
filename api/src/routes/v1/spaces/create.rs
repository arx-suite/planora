use actix_web::{HttpRequest, Responder, post, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::db::{dto::space::NewSpace, repos::SpaceRepo};
use arx_gatehouse::services::DbManager;

#[post("")]
async fn create_space(
    manager: web::Data<DbManager>,
    payload: web::Json<NewSpace>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let new_space = payload.into_inner();
    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%new_space.space_name, %org_id, "create space for organization");

    let space_repo = SpaceRepo::new(&pool);
    let inserted_space = space_repo.create_space(new_space, org_id).await?;

    tracing::info!(%inserted_space.space_id, %org_id, "space created successfully");
    ApiResult::to_ok_response("space has been created successfully", inserted_space)
}
