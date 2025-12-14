use sea_query::*;
use sqlx::PgPool;

use super::{CreateOrg, OrganizationRow, Organizations};
use crate::db::DBResult;

pub struct OrgRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> OrgRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_org(
        &self,
        org: &CreateOrg,
        owner_id: uuid::Uuid,
    ) -> DBResult<OrganizationRow> {
        let query = Query::insert()
            .into_table(Organizations::Table)
            .columns([
                Organizations::OwnerId,
                Organizations::Name,
                Organizations::Subdomain,
            ])
            .values_panic([
                owner_id.into(),
                org.name.to_owned().into(),
                org.subdomain.to_owned().into(),
            ])
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let inserted_org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_one(self.pool)
            .await?;

        Ok(inserted_org)
    }

    pub async fn find_by_ownerid(&self, owner_id: uuid::Uuid) -> DBResult<Vec<OrganizationRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::OwnerId).eq(owner_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_all(self.pool)
            .await?;
        Ok(org)
    }

    pub async fn find_by_orgid(&self, org_id: uuid::Uuid) -> DBResult<Option<OrganizationRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Organizations::Table)
            .and_where(Expr::col(Organizations::OwnerId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, OrganizationRow>(&query)
            .fetch_optional(self.pool)
            .await?;
        Ok(org)
    }

    pub async fn delete_by_orgid(&self, org_id: uuid::Uuid) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_subdomain(&self, subdomain: String) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Organizations::Table)
            .and_where(Expr::col(Organizations::OrganizationId).eq(subdomain))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;
        Ok(result.rows_affected())
    }
}
