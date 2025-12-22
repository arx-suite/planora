use actix_web::{HttpRequest, Responder, post, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::space::{NewSpace, SpaceInfo, SpaceRepo};
use arx_gatehouse::services::DbService;

#[post("")]
async fn create_space(
    db_service: web::Data<DbService>,
    payload: web::Json<NewSpace>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let new_space = payload.into_inner();
    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%new_space.space_name, %org_id, "create space for organization");

    let inserted_space: SpaceInfo = pool.space_create(new_space, org_id).await?.into();

    tracing::info!(%inserted_space.space_id, %org_id, "space created successfully");
    ApiResult::to_ok_response("space has been created successfully", inserted_space)
}

/*
#[post("")]
#[instrument(name = "create_space", skip(manager, payload, req), fields(org_id))]
async fn create_space(
    manager: web::Data<DbManager>,
    payload: web::Json<NewSpace>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    // Extract payload
    let new_space = payload.into_inner();

    // Top-level trace: fetching DB pool
    let pool_span = info_span!("get_db_pool");
    let pool = {
        let _enter = pool_span.enter();
        manager.get_planora_pool().await?
    };

    // Extract org_id
    let org_id = extract_org_id(&req)?;
    trace!(%org_id, "organization id extracted");

    // Validate org
    let validate_span = info_span!("validate_org");
    {
        let _enter = validate_span.enter();
        validate_org(&pool, org_id).await?;
    }

    // Log before DB operation
    trace!(%new_space.space_name, %org_id, "creating space in DB");

    // Nested DB span
    let db_span = info_span!("db_insert_space");
    let inserted_space: SpaceInfo = {
        let _enter = db_span.enter();
        let space_repo = SpaceRepo::new(&pool);
        space_repo.create_space(new_space, org_id).await?.into()
    };

    info!(%inserted_space.space_id, %org_id, "space created successfully");

    ApiResult::to_ok_response("space has been created successfully", inserted_space)
}
*/
