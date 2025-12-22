use actix_web::{HttpRequest, Responder, get, web};

use super::helper::validate_org;
use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_org_id};
use arx_gatehouse::modules::project::{ProjectInfo, ProjectRepo};
use arx_gatehouse::services::DbService;

#[get("")]
async fn list_projects(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let pool = db_service.primary().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, "Listing projects for organization");

    let project_repo = ProjectRepo::new(&pool);
    let projects = project_repo.find_by_orgid(org_id).await?;

    let projects = projects
        .into_iter()
        .map(|org| org.into())
        .collect::<Vec<ProjectInfo>>();

    tracing::info!(%org_id, len = projects.len(), "Projects listed successfully");

    if projects.len() == 0 {
        return ApiResult::to_no_content("No projects");
    } else {
        return ApiResult::to_ok_response("projects", projects);
    }
}
