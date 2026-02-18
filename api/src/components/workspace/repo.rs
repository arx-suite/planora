#![allow(unused_variables)]

use sea_query::*;
use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{
    Features, OrganizationFeatureRow, OrganizationFeatures, OrganizationResourceRow,
    OrganizationRow, Organizations, Plans,
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
    async fn organization_find_by_subdomain(
        &self,
        subdomain: String,
    ) -> DBResult<Option<OrganizationRow>>;
    async fn organization_find_by_id(&self, org_id: Uuid) -> DBResult<Option<OrganizationRow>>;
    async fn organization_update(
        &self,
        org_id: Uuid,
        plan: Option<String>,
    ) -> DBResult<OrganizationRow>;
    async fn organization_delete(&self, org_id: Uuid) -> DBResult<()>;

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

    async fn organization_find_by_subdomain(
        &self,
        subdomain: String,
    ) -> DBResult<Option<OrganizationRow>> {
        let stmt = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::Subdomain).eq(subdomain))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&stmt)
            .fetch_optional(self)
            .await?;

        Ok(org)
    }

    async fn organization_find_by_id(&self, org_id: Uuid) -> DBResult<Option<OrganizationRow>> {
        let stmt = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(org_id))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&stmt)
            .fetch_optional(self)
            .await?;

        Ok(org)
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
        let enabled_alias = Alias::new("enabled");

        let query = Query::select()
            .column((Features::Table, Features::FeatureName))
            .column((Features::Table, Features::Description))
            .expr_as(
                Func::coalesce([
                    Expr::col((OrganizationFeatures::Table, OrganizationFeatures::Enabled)).into(),
                    Expr::col((Features::Table, Features::DefaultEnabled)).into(),
                ]),
                enabled_alias,
            )
            .from(Organizations::Table)
            .join(
                sea_query::JoinType::Join,
                Plans::Table,
                Expr::col((Plans::Table, Plans::PlanName))
                    .equals((Organizations::Table, Organizations::Plan)),
            )
            .join(
                sea_query::JoinType::Join,
                Features::Table,
                Expr::col((Features::Table, Features::MinPlanLevel))
                    .lte(Expr::col((Plans::Table, Plans::PlanLevel))),
            )
            .join(
                sea_query::JoinType::LeftJoin,
                OrganizationFeatures::Table,
                Expr::col((
                    OrganizationFeatures::Table,
                    OrganizationFeatures::OrganizationId,
                ))
                .equals((Organizations::Table, Organizations::OrganizationId))
                .and(
                    Expr::col((
                        OrganizationFeatures::Table,
                        OrganizationFeatures::FeatureName,
                    ))
                    .equals((Features::Table, Features::FeatureName)),
                ),
            )
            .and_where(Expr::col((Organizations::Table, Organizations::OrganizationId)).eq(org_id))
            .order_by(
                (Features::Table, Features::FeatureName),
                sea_query::Order::Asc,
            )
            .to_string(PostgresQueryBuilder);

        let features = sqlx::query_as::<_, OrganizationFeatureRow>(&query)
            .fetch_all(self)
            .await?;

        Ok(features)
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
