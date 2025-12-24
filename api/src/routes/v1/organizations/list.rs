use actix_web::{HttpRequest, Responder, get, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::modules::organization::{OrgProfile, OrgRepo};
use arx_gatehouse::services::DbService;

#[get("")]
#[tracing::instrument(
    name = "org.get",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        user_id = tracing::field::Empty
    )
)]
async fn list_organizations(
    db_service: web::Data<DbService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::Span::current().record("user_id", &user_id.to_string());

    let pool = db_service.read().await?;
    tracing::debug!("database pool acquired");

    let orgs = pool.org_find_by_owner_id(user_id).await?;

    let orgs = orgs
        .into_iter()
        .map(|org| org.into())
        .collect::<Vec<OrgProfile>>();

    tracing::info!(len = %orgs.len(), "organization listed successfully");

    ApiResult::to_ok_response("organization list", orgs)
}
