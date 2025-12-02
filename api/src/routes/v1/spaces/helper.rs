use arx_gatehouse::{common::ApiError, db::repos::OrgRepo};

pub async fn validate_org(pool: &sqlx::PgPool, org_id: uuid::Uuid) -> Result<(), ApiError> {
    let repo = OrgRepo::new(pool);
    tracing::trace!(%org_id, "Validating organization existence");

    match repo.find_by_orgid(org_id).await {
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
