use arx_gatehouse::common::ApiError;
use arx_gatehouse::modules::organization::OrgRepo;

pub async fn validate_org(pool: &sqlx::PgPool, org_id: uuid::Uuid) -> Result<(), ApiError> {
    tracing::trace!(%org_id, "Validating organization existence");

    match pool.org_find_by_id(org_id).await {
        Ok(Some(_)) => {
            tracing::info!(%org_id, "Organization verified");
            Ok(())
        }
        Ok(None) => {
            tracing::warn!(%org_id, "Unauthorized access: organization not found");
            Err(ApiError::Unauthorized(
                "You are not allowed to perform this action".into(),
            ))
        }
        Err(e) => {
            tracing::error!(error = ?e, %org_id, "Database error while validating org");
            Err(ApiError::Internal("Failed to verify organization".into()))
        }
    }
}
