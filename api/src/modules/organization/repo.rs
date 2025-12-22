use sea_query::*;
use sqlx::PgPool;

use super::{CreateOrg, OrganizationRow, Organizations};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait OrgRepo {
    async fn org_create(&self, org: &CreateOrg, owner_id: uuid::Uuid) -> DBResult<OrganizationRow>;
    async fn org_find_by_id(&self, org_id: uuid::Uuid) -> DBResult<Option<OrganizationRow>>;
    async fn org_find_by_owner_id(&self, owner_id: uuid::Uuid) -> DBResult<Vec<OrganizationRow>>;
    async fn org_delete_by_id(&self, org_id: uuid::Uuid) -> DBResult<u64>;
    async fn org_delete_by_subdomain(&self, subdomain: String) -> DBResult<u64>;
}

#[async_trait::async_trait]
impl OrgRepo for PgPool {
    async fn org_create(&self, org: &CreateOrg, owner_id: uuid::Uuid) -> DBResult<OrganizationRow> {
        let query = Query::insert()
            .into_table(Organizations::Table)
            .columns([
                Organizations::OwnerId,
                Organizations::Name,
                Organizations::Subdomain,
            ])
            .values([
                owner_id.into(),
                org.name.to_owned().into(),
                org.subdomain.to_owned().into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let inserted_org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_one(self)
            .await?;

        Ok(inserted_org)
    }

    async fn org_find_by_id(&self, org_id: uuid::Uuid) -> DBResult<Option<OrganizationRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::OwnerId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(org)
    }

    async fn org_find_by_owner_id(&self, owner_id: uuid::Uuid) -> DBResult<Vec<OrganizationRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::OwnerId).eq(owner_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_all(self)
            .await?;

        Ok(org)
    }

    async fn org_delete_by_id(&self, org_id: uuid::Uuid) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self).await?;

        Ok(result.rows_affected())
    }

    async fn org_delete_by_subdomain(&self, subdomain: String) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(subdomain))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self).await?;

        Ok(result.rows_affected())
    }
}
