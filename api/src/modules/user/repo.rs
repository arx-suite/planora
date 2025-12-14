use sea_query::*;
use sqlx::PgPool;
use tracing::info;

use super::{CreateUser, UserRow, Users};
use crate::db::DBResult;

pub struct UserRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /* create */
    pub async fn create_user(&self, user: CreateUser) -> DBResult<UserRow> {
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

        let created_user = sqlx::query_as::<_, UserRow>(&query)
            .fetch_one(self.pool)
            .await?;

        info!(
            action = "user_creation",
            %created_user.user_id,
            "user created"
        );

        Ok(created_user)
    }

    /* read */
    pub async fn find_by_email(&self, email: String) -> DBResult<Option<UserRow>> {
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
            .fetch_optional(self.pool)
            .await?;

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    pub async fn find_by_userid(&self, userid: uuid::Uuid) -> DBResult<Option<UserRow>> {
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
            .fetch_optional(self.pool)
            .await?;

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    pub async fn find_by_usertag(&self, usertag: String) -> DBResult<Option<UserRow>> {
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
            .fetch_optional(self.pool)
            .await?;

        info!(
            action = "user_lookup",
            found = user.is_some(),
            "finished user lookup"
        );

        Ok(user)
    }

    /* delete */
    pub async fn delete_by_email(&self, email: String) -> DBResult<u64> {
        info!(
            action = "user_deletion",
            %email,
            "deleting user"
        );

        let query = Query::delete()
            .from_table(Users::Table)
            .and_where(Expr::col(Users::Email).eq(email))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;

        info!(
            action = "user_deletion",
            deleted = result.rows_affected() > 0,
            "deleting user"
        );

        Ok(result.rows_affected())
    }

    pub async fn list_users(&self, limit: u64, offset: u64) -> DBResult<Vec<UserRow>> {
        info!(
            action = "user_listing",
            %limit,
            %offset,
            "listing users"
        );

        let query = Query::select()
            .column(Asterisk)
            .from(Users::Table)
            .limit(limit)
            .offset(offset)
            .order_by(Users::CreatedAt, Order::Desc)
            .to_string(PostgresQueryBuilder);

        let users = sqlx::query_as::<_, UserRow>(&query)
            .fetch_all(self.pool)
            .await?;

        info!(action = "user_listing", "user listed");

        Ok(users)
    }
}
