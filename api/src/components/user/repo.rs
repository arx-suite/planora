use sea_query::*;
use sqlx::PgExecutor;

use super::model::{UserRow, UserStatus, Users};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait UserRepo {
    async fn user_create_email(
        &self,
        username: String,
        email: String,
        password: String,
        usertag: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> DBResult<UserRow>;
    async fn user_find_by_email(&self, email: String) -> DBResult<Option<UserRow>>;
    async fn user_find_by_usertag(&self, usertag: String) -> DBResult<Option<UserRow>>;
    async fn user_find_by_id(&self, userid: uuid::Uuid) -> DBResult<Option<UserRow>>;
}

#[async_trait::async_trait]
impl<T> UserRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    // create
    async fn user_create_email(
        &self,
        username: String,
        email: String,
        password: String,
        usertag: String,
        created_at: chrono::DateTime<chrono::Utc>,
    ) -> DBResult<UserRow> {
        tracing::debug!(
            action = "user_creation",
            %username,
            "creating new user"
        );

        let query = Query::insert()
            .columns([
                Users::Status,
                Users::EmailVerifiedAt,
                Users::Usertag,
                Users::Username,
                Users::Email,
                Users::PasswordHash,
                Users::CreatedAt,
            ])
            .values([
                UserStatus::Active.into(),
                Expr::current_timestamp().into(),
                usertag.into(),
                username.into(),
                email.into(),
                password.into(),
                created_at.into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let created_user = sqlx::query_as::<_, UserRow>(&query).fetch_one(self).await?;

        tracing::info!(
            action = "user_creation",
            %created_user.user_id,
            "user created"
        );

        Ok(created_user)
    }

    // read
    async fn user_find_by_email(&self, email: String) -> DBResult<Option<UserRow>> {
        tracing::debug!(
            action = "user_lookup",
            lookup_field = "email",
            %email,
            "starting user lookup"
        );

        let query = Query::select()
            .column(Asterisk)
            .from(Users::Table)
            .and_where(Expr::col(Users::Email).eq(email))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, UserRow>(&query)
            .fetch_optional(self)
            .await?;

        tracing::debug!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    async fn user_find_by_usertag(&self, usertag: String) -> DBResult<Option<UserRow>> {
        tracing::debug!(
            action = "user_lookup",
            lookup_field = "usertag",
            %usertag,
            "starting user lookup"
        );

        let query = Query::select()
            .column(Asterisk)
            .from(Users::Table)
            .and_where(Expr::col(Users::Usertag).eq(usertag))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, UserRow>(&query)
            .fetch_optional(self)
            .await?;

        tracing::debug!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    async fn user_find_by_id(&self, userid: uuid::Uuid) -> DBResult<Option<UserRow>> {
        tracing::debug!(
            action = "user_lookup",
            lookup_field = "userid",
            %userid,
            "starting user lookup"
        );

        let query = Query::select()
            .column(Asterisk)
            .from(Users::Table)
            .and_where(Expr::col(Users::UserId).eq(userid.to_string()))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, UserRow>(&query)
            .fetch_optional(self)
            .await?;

        tracing::debug!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }
}
