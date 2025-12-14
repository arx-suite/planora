use actix_web::{HttpRequest, Responder, get, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::project::ProjectRepo;
use arx_gatehouse::services::DbManager;

#[get("")]
async fn list_projects(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, "Listing projects for organization");

    let project_repo = ProjectRepo::new(&pool);
    let projects = project_repo.find_by_orgid(org_id).await?;

    tracing::info!(%org_id, len = projects.len(), "Projects listed successfully");

    if projects.len() == 0 {
        return ApiResult::to_no_content("No projects");
    } else {
        return ApiResult::to_ok_response("projects", projects);
    }
}
