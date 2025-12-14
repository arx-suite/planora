use actix_web::{HttpRequest, Responder, post, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::project::{CreateProject, ProjectInfo, ProjectRepo};
use arx_gatehouse::services::DbManager;

#[post("")]
async fn create_project(
    manager: web::Data<DbManager>,
    payload: web::Json<CreateProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let project = payload.into_inner();

    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%project.name, %org_id, "create project for organization");

    let project_repo = ProjectRepo::new(&pool);

    let inserted_project: ProjectInfo = project_repo.create_project(&project, org_id).await?.into();

    tracing::info!(%inserted_project.project_id, %org_id, "project created successfully");

    ApiResult::to_ok_response("project has been created successfully", inserted_project)
}
