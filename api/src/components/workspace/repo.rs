#![allow(unused_variables)]

use sea_query::*;
use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{
    OrganizationFeatureRow, OrganizationResourceRow, OrganizationRow, Organizations,
};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait WorkspaceRepo {
    // organizations
    async fn organization_create(
        &self,
        created_by: Uuid,
        name: String,
        subdomain: String,
    ) -> DBResult<OrganizationRow>;
    async fn organization_delete(&self, org_id: Uuid) -> DBResult<()>;
    async fn organization_update(
        &self,
        org_id: Uuid,
        plan: Option<String>,
    ) -> DBResult<OrganizationRow>;

    // resources
    async fn resources(&self, org_id: Uuid) -> DBResult<OrganizationResourceRow>;

    // features
    async fn features(&self, org_id: Uuid) -> DBResult<Vec<OrganizationFeatureRow>>;
    async fn feature_enable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()>;
    async fn feature_disable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()>;
    async fn upgrade_plan(&self, org_id: Uuid) -> DBResult<()>;
}

#[async_trait::async_trait]
impl<T> WorkspaceRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    async fn organization_create(
        &self,
        created_by: Uuid,
        name: String,
        subdomain: String,
    ) -> DBResult<OrganizationRow> {
        let stmt = Query::insert()
            .into_table(Organizations::Table)
            .columns([
                Organizations::CreatedBy,
                Organizations::Name,
                Organizations::Subdomain,
            ])
            .values([created_by.into(), name.into(), subdomain.into()])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, OrganizationRow>(&stmt)
            .fetch_one(self)
            .await?;

        Ok(user)
    }

    async fn organization_delete(&self, org_id: Uuid) -> DBResult<()> {
        let stmt = Query::delete()
            .from_table(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(org_id))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&stmt).execute(self).await?;

        if result.rows_affected() != 1 {
            tracing::error!(
                action = "organization_delete",
                expected = 1,
                actual = result.rows_affected(),
                "unexpected number of rows affected while deleting organization"
            );
        }

        Ok(())
    }

    async fn organization_update(
        &self,
        org_id: Uuid,
        plan: Option<String>,
    ) -> DBResult<OrganizationRow> {
        let stmt = {
            let mut stmt = Query::update();
            stmt.table(Organizations::Table)
                .and_where(Expr::col(Organizations::OrganizationId).eq(org_id));

            if let Some(plan) = plan {
                stmt.value(Organizations::Plan, plan);
            };

            stmt.value(Organizations::UpdatedAt, Expr::current_timestamp())
                .returning_all();

            stmt.to_string(PostgresQueryBuilder)
        };

        let organization = sqlx::query_as::<_, OrganizationRow>(&stmt)
            .fetch_one(self)
            .await?;

        Ok(organization)
    }

    async fn resources(&self, org_id: Uuid) -> DBResult<OrganizationResourceRow> {
        todo!()
    }

    async fn features(&self, org_id: Uuid) -> DBResult<Vec<OrganizationFeatureRow>> {
        todo!()
    }

    async fn feature_enable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()> {
        todo!()
    }

    async fn feature_disable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()> {
        todo!()
    }

    async fn upgrade_plan(&self, org_id: Uuid) -> DBResult<()> {
        todo!()
    }
}
