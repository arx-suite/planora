use sea_query::*;
use sqlx::PgPool;

use super::{CreateProject, DeleteProject, ProjectRow, Projects};
use crate::services::db::{DBResult, TenantContext};

#[async_trait::async_trait]
pub trait ProjectRepo {
    async fn project_create(
        &self,
        project: &CreateProject,
        org_id: uuid::Uuid,
    ) -> DBResult<ProjectRow>;

    async fn project_find_by_id(
        &self,
        project_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> DBResult<Option<ProjectRow>>;

    async fn project_find_by_org_id(&self, org_id: uuid::Uuid) -> DBResult<Vec<ProjectRow>>;

    async fn project_delete_by_id(
        &self,
        project: DeleteProject,
        org_id: uuid::Uuid,
    ) -> DBResult<u64>;
}

#[async_trait::async_trait]
impl ProjectRepo for PgPool {
    async fn project_create(
        &self,
        project: &CreateProject,
        org_id: uuid::Uuid,
    ) -> DBResult<ProjectRow> {
        let query = Query::insert()
            .into_table(Projects::Table)
            .columns([
                Projects::OrganizationId,
                Projects::Name,
                Projects::Description,
            ])
            .values([
                org_id.into(),
                project.name.clone().into(),
                project.description.clone().into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let inserted_project = Self::with_tenant_context(self, &org_id, |mut tx| async move {
            let inserted = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_one(&mut *tx)
                .await?;

            Ok((inserted, tx))
        })
        .await?;

        Ok(inserted_project)
    }

    async fn project_find_by_id(
        &self,
        project_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> DBResult<Option<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::ProjectId).eq(project_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let project = Self::with_tenant_context(self, &org_id, |mut tx| async move {
            let project = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_optional(&mut *tx)
                .await?;

            Ok((project, tx))
        })
        .await?;

        Ok(project)
    }

    async fn project_find_by_org_id(&self, org_id: uuid::Uuid) -> DBResult<Vec<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::OrganizationId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let projects = Self::with_tenant_context(self, &org_id, |mut tx| async move {
            let projects = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_all(&mut *tx)
                .await?;

            Ok((projects, tx))
        })
        .await?;

        Ok(projects)
    }

    async fn project_delete_by_id(
        &self,
        project: DeleteProject,
        org_id: uuid::Uuid,
    ) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Projects::Table)
            .and_where(Expr::col(Projects::ProjectId).eq(project.project_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let result = Self::with_tenant_context(self, &org_id, |mut tx| async move {
            let result = sqlx::query(&query).execute(&mut *tx).await?;
            Ok((result, tx))
        })
        .await?;

        Ok(result.rows_affected())
    }
}
