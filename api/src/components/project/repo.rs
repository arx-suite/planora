use chrono::{DateTime, Utc};
use sea_query::*;
use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{ProjectPriority, ProjectRow, ProjectStatus, ProjectVisibility, Projects};
use super::model::{TaskPriority, TaskRow, TaskStatus, Tasks};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait ProjectRepo {
    async fn project_create(
        &self,
        organization_id: Uuid,
        name: String,
        description: Option<String>,
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
pub trait TaskRepo {
    async fn task_create(
        &self,
        project_id: Uuid,
        name: String,
        description: Option<String>,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
        start_date: Option<DateTime<Utc>>,
        due_date: Option<DateTime<Utc>>,
    ) -> DBResult<TaskRow>;
    async fn task_find_by_id(&self, id: Uuid) -> DBResult<Option<TaskRow>>;
    async fn task_find_by_name(&self, name: String) -> DBResult<Option<TaskRow>>;
    async fn task_find_by_key(&self, key: String) -> DBResult<Option<TaskRow>>;
    async fn tasks_for_project(&self, project_id: Uuid) -> DBResult<Vec<TaskRow>>;
    async fn task_update(&self) -> DBResult<TaskRow>;
    async fn task_delete_by_key(&self, key: String) -> DBResult<u64>;
    async fn task_delete_by_name(&self, name: String) -> DBResult<u64>;
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
        description: Option<String>,
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

#[async_trait::async_trait]
impl<T> TaskRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    async fn task_create(
        &self,
        project_id: Uuid,
        name: String,
        description: Option<String>,
        status: Option<TaskStatus>,
        priority: Option<TaskPriority>,
        start_date: Option<DateTime<Utc>>,
        due_date: Option<DateTime<Utc>>,
    ) -> DBResult<TaskRow> {
        let query = Query::insert()
            .into_table(Tasks::Table)
            .columns([
                Tasks::ProjectId,
                Tasks::TaskName,
                Tasks::Description,
                Tasks::Status,
                Tasks::Priority,
                Tasks::StartDate,
                Tasks::DueDate,
            ])
            .values([
                project_id.into(),
                name.into(),
                description.into(),
                status.unwrap_or_default().to_string().into(),
                priority.unwrap_or_default().to_string().into(),
                start_date.into(),
                due_date.into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let created_task = sqlx::query_as::<_, TaskRow>(&query).fetch_one(self).await?;

        Ok(created_task)
    }

    async fn task_find_by_id(&self, id: Uuid) -> DBResult<Option<TaskRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Tasks::Table)
            .and_where(Expr::col(Tasks::TaskId).eq(id))
            .to_string(PostgresQueryBuilder);

        let task = sqlx::query_as::<_, TaskRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(task)
    }

    async fn task_find_by_name(&self, name: String) -> DBResult<Option<TaskRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Tasks::Table)
            .and_where(Expr::col(Tasks::TaskName).eq(name))
            .to_string(PostgresQueryBuilder);

        let task = sqlx::query_as::<_, TaskRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(task)
    }

    async fn task_find_by_key(&self, key: String) -> DBResult<Option<TaskRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Tasks::Table)
            .and_where(Expr::col(Tasks::TaskKey).eq(key))
            .to_string(PostgresQueryBuilder);

        let task = sqlx::query_as::<_, TaskRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(task)
    }

    async fn tasks_for_project(&self, project_id: Uuid) -> DBResult<Vec<TaskRow>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Tasks::Table)
            .and_where(Expr::col(Tasks::ProjectId).eq(project_id))
            .to_string(PostgresQueryBuilder);

        let tasks = sqlx::query_as::<_, TaskRow>(&query).fetch_all(self).await?;

        Ok(tasks)
    }

    async fn task_update(&self) -> DBResult<TaskRow> {
        let query = {
            let mut query = Query::update();

            query
                .value(Tasks::UpdatedAt, Expr::current_timestamp())
                .returning_all();

            query.to_string(PostgresQueryBuilder)
        };

        let task = sqlx::query_as::<_, TaskRow>(&query).fetch_one(self).await?;

        Ok(task)
    }

    async fn task_delete_by_key(&self, key: String) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Tasks::Table)
            .and_where(Expr::col(Tasks::TaskKey).eq(key))
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self).await?;

        Ok(result.rows_affected())
    }

    async fn task_delete_by_name(&self, name: String) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Tasks::Table)
            .and_where(Expr::col(Tasks::TaskName).eq(name))
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self).await?;

        Ok(result.rows_affected())
    }
}
