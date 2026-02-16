use chrono::{DateTime, Utc};
use sea_query::*;
use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{ProjectPriority, ProjectRow, ProjectStatus, ProjectVisibility, Projects};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait ProjectRepo {
    async fn project_create(
        &self,
        organization_id: Uuid,
        name: String,
        description: String,
        tags: Vec<String>,
        status: Option<ProjectStatus>,
        priority: Option<ProjectVisibility>,
        start_date: Option<DateTime<Utc>>,
        target_date: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> DBResult<ProjectRow>;

    async fn project_find_by_id(&self, project_id: Uuid) -> DBResult<Option<ProjectRow>>;
    async fn project_find_by_name(
        &self,
        project_name: String,
        organization_id: Uuid,
    ) -> DBResult<Option<ProjectRow>>;

    async fn projects_for_org(
        &self,
        organization_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> DBResult<Vec<ProjectRow>>;

    async fn project_update(
        &self,
        project_id: Uuid,
        organization_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        priority: Option<ProjectPriority>,
        status: Option<ProjectStatus>,
        visibility: Option<ProjectVisibility>,
    ) -> DBResult<ProjectRow>;
}

#[async_trait::async_trait]
impl<T> ProjectRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    async fn project_create(
        &self,
        organization_id: Uuid,
        name: String,
        description: String,
        tags: Vec<String>,
        status: Option<ProjectStatus>,
        visibility: Option<ProjectVisibility>,
        start_date: Option<DateTime<Utc>>,
        target_date: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> DBResult<ProjectRow> {
        let query = Query::insert()
            .into_table(Projects::Table)
            .columns([
                Projects::OrganizationId,
                Projects::ProjectName,
                Projects::Description,
                Projects::Tags,
                Projects::Status,
                Projects::Visibility,
                Projects::StartDate,
                Projects::TargetDate,
                Projects::CreatedBy,
            ])
            .values([
                organization_id.into(),
                name.into(),
                description.into(),
                tags.into(),
                status.unwrap_or_default().into(),
                visibility.unwrap_or_default().into(),
                start_date.into(),
                target_date.into(),
                created_by.into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let created_project = sqlx::query_as::<_, ProjectRow>(&query)
            .fetch_one(self)
            .await?;

        Ok(created_project)
    }

    async fn project_find_by_id(&self, project_id: Uuid) -> DBResult<Option<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::ProjectId).eq(project_id))
            .to_string(PostgresQueryBuilder);

        let project = sqlx::query_as::<_, ProjectRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(project)
    }

    async fn project_find_by_name(
        &self,
        project_name: String,
        organization_id: Uuid,
    ) -> DBResult<Option<ProjectRow>> {
        let query = Query::select()
            .from(Projects::Table)
            .column(Asterisk)
            .and_where(Expr::col(Projects::ProjectName).eq(project_name))
            .and_where(Expr::col(Projects::OrganizationId).eq(organization_id))
            .to_string(PostgresQueryBuilder);

        let project = sqlx::query_as::<_, ProjectRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(project)
    }

    async fn projects_for_org(
        &self,
        organization_id: Uuid,
        limit: u64,
        offset: u64,
    ) -> DBResult<Vec<ProjectRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Projects::Table)
            .and_where(Expr::col(Projects::OrganizationId).eq(organization_id))
            .limit(limit)
            .offset(offset)
            .to_string(PostgresQueryBuilder);

        let projects = sqlx::query_as::<_, ProjectRow>(&query)
            .fetch_all(self)
            .await?;

        Ok(projects)
    }

    async fn project_update(
        &self,
        project_id: Uuid,
        organization_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        priority: Option<ProjectPriority>,
        status: Option<ProjectStatus>,
        visibility: Option<ProjectVisibility>,
    ) -> DBResult<ProjectRow> {
        let query = {
            let mut query = Query::update();
            query
                .table(Projects::Table)
                .and_where(Expr::col(Projects::ProjectId).eq(project_id))
                .and_where(Expr::col(Projects::OrganizationId).eq(organization_id));

            if let Some(name) = name {
                query.value(Projects::ProjectName, name);
            }
            if let Some(desc) = description {
                query.value(Projects::Description, desc);
            }
            if let Some(tags) = tags {
                query.value(Projects::Tags, tags);
            }
            if let Some(priority) = priority {
                query.value(Projects::Priority, priority);
            }
            if let Some(visibility) = visibility {
                query.value(Projects::Visibility, visibility);
            }
            if let Some(status) = status {
                query.value(Projects::Status, status);
            }

            /* TODO
            if query.get_values().is_empty() {
                return Err(DatabaseError::NoFieldsToUpdate);
            }
            */

            query
                .value(Projects::UpdatedAt, Expr::current_timestamp())
                .returning_all();

            query.to_string(PostgresQueryBuilder)
        };

        let project = sqlx::query_as::<_, ProjectRow>(&query)
            .fetch_one(self)
            .await?;

        Ok(project)
    }
}
