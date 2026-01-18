use sea_query::*;
use sqlx::PgExecutor;
use tracing::info;

use super::{CreateUser, UserRow, Users};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait UserRepo {
    async fn user_create(&self, user: CreateUser) -> DBResult<UserRow>;
    async fn user_find_by_id(&self, userid: uuid::Uuid) -> DBResult<Option<UserRow>>;
    async fn user_find_by_email(&self, email: String) -> DBResult<Option<UserRow>>;
    async fn user_find_by_tag(&self, usertag: String) -> DBResult<Option<UserRow>>;
    async fn user_delete_by_email(&self, email: String) -> DBResult<u64>;
}

#[async_trait::async_trait]
impl<T> UserRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    async fn user_create(&self, user: CreateUser) -> DBResult<UserRow> {
        info!(
            action = "user_creation",
            %user.username,
            "creating new user"
        );

        let query = Query::insert()
            .into_table(Users::Table)
            .columns([Users::Username, Users::Email, Users::Password])
            .values([
                user.username.into(),
                user.email.into(),
                user.password.into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let created_user = sqlx::query_as::<_, UserRow>(&query).fetch_one(self).await?;

        info!(
            action = "user_creation",
            %created_user.user_id,
            "user created"
        );

        Ok(created_user)
    }

    async fn user_find_by_id(&self, userid: uuid::Uuid) -> DBResult<Option<UserRow>> {
        info!(
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

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    async fn user_find_by_email(&self, email: String) -> DBResult<Option<UserRow>> {
        info!(
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

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    async fn user_find_by_tag(&self, usertag: String) -> DBResult<Option<UserRow>> {
        info!(
            action = "user_lookup",
            lookup_field = "usertag",
            %usertag,
            "starting user lookup"
        );

        let query = Query::select()
            .column(Asterisk)
            .from(Users::Table)
            .and_where(Expr::col(Users::UserTag).eq(usertag))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, UserRow>(&query)
            .fetch_optional(self)
            .await?;

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    async fn user_delete_by_email(&self, email: String) -> DBResult<u64> {
        info!(
            action = "user_deletion",
            %email,
            "deleting user"
        );

        let query = Query::delete()
            .from_table(Users::Table)
            .and_where(Expr::col(Users::Email).eq(email))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self).await?;

        info!(
            action = "user_deletion",
            deleted = result.rows_affected() > 0,
            "deleting user"
        );

        Ok(result.rows_affected())
    }
}
