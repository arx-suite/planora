use sea_query::*;
use sqlx::PgPool;

use super::{CreateProject, DeleteProject, ProjectRow, Projects};
use crate::services::db::{DBResult, with_org};

pub struct ProjectRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> ProjectRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_project(
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

        let inserted_project = with_org(self.pool, &org_id, |mut tx| async move {
            let inserted = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_one(&mut *tx)
                .await?;

            Ok((inserted, tx))
        })
        .await?;

        Ok(inserted_project)
    }

    pub async fn find_by_projectid(
        &self,
        project_id: uuid::Uuid,
        org_id: uuid::Uuid,
    ) -> DBResult<Option<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::ProjectId).eq(project_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let project = with_org(self.pool, &org_id, |mut tx| async move {
            let project = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_optional(&mut *tx)
                .await?;

            Ok((project, tx))
        })
        .await?;

        Ok(project)
    }

    pub async fn find_by_orgid(&self, org_id: uuid::Uuid) -> DBResult<Vec<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::OrganizationId).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let projects = with_org(self.pool, &org_id, |mut tx| async move {
            let projects = sqlx::query_as::<_, ProjectRow>(&query)
                .fetch_all(&mut *tx)
                .await?;

            Ok((projects, tx))
        })
        .await?;

        Ok(projects)
    }

    pub async fn delete_by_projectid(
        &self,
        project: DeleteProject,
        org_id: uuid::Uuid,
    ) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Projects::Table)
            .and_where(Expr::col(Projects::ProjectId).eq(project.project_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let result = with_org(self.pool, &org_id, |mut tx| async move {
            let result = sqlx::query(&query).execute(&mut *tx).await?;
            Ok((result, tx))
        })
        .await?;

        Ok(result.rows_affected())
    }
}
