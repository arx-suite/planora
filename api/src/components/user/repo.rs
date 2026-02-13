use sea_query::*;
use sqlx::PgExecutor;

use super::model::{UserRow, UserSessionRow, UserSessions, UserStatus, Users};
use crate::{components::user::model::SessionStatus, services::db::DBResult};

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

    async fn session_create(
        &self,
        userid: uuid::Uuid,
        user_agent: String,
        ip_addr: Option<sqlx::types::ipnetwork::IpNetwork>,
        ip_country: Option<String>,
        device_type: Option<String>,
        device_name: Option<String>,
        os_name: Option<String>,
        access_expired_at: chrono::DateTime<chrono::Utc>,
        refresh_expired_at: chrono::DateTime<chrono::Utc>,
    ) -> DBResult<UserSessionRow>;
    async fn session_find_by_id(&self, session_id: uuid::Uuid) -> DBResult<Option<UserSessionRow>>;
    async fn session_find_by_userid(&self, userid: uuid::Uuid) -> DBResult<Vec<UserSessionRow>>;
    async fn session_revoke(&self, session_id: uuid::Uuid, reason: &str) -> DBResult<()>;
}

#[async_trait::async_trait]
impl<T> UserRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
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

    // session
    async fn session_create(
        &self,
        userid: uuid::Uuid,
        user_agent: String,
        ip_addr: Option<sqlx::types::ipnetwork::IpNetwork>,
        ip_country: Option<String>,
        device_type: Option<String>,
        device_name: Option<String>,
        os_name: Option<String>,
        access_expired_at: chrono::DateTime<chrono::Utc>,
        refresh_expired_at: chrono::DateTime<chrono::Utc>,
    ) -> DBResult<UserSessionRow> {
        tracing::debug!(user_id = %userid, "creating session");

        let query = Query::insert()
            .into_table(UserSessions::Table)
            .columns([
                UserSessions::UserId,
                UserSessions::UserAgent,
                UserSessions::IpAddress,
                UserSessions::IpCountry,
                UserSessions::DeviceType,
                UserSessions::DeviceName,
                UserSessions::OsName,
                UserSessions::Status,
                UserSessions::AccessExpiredAt,
                UserSessions::RefreshExpiredAt,
            ])
            .values([
                userid.into(),
                user_agent.into(),
                ip_addr.into(),
                ip_country.into(),
                device_type.into(),
                device_name.into(),
                os_name.into(),
                SessionStatus::Active.to_string().into(),
                access_expired_at.into(),
                refresh_expired_at.into(),
            ])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let row = sqlx::query_as::<_, UserSessionRow>(&query)
            .fetch_one(self)
            .await?;

        Ok(row)
    }

    async fn session_find_by_id(&self, session_id: uuid::Uuid) -> DBResult<Option<UserSessionRow>> {
        tracing::debug!(%session_id, "looking up session");

        let query = Query::select()
            .column(Asterisk)
            .from(UserSessions::Table)
            .and_where(Expr::col(UserSessions::SessionId).eq(session_id))
            .to_string(PostgresQueryBuilder);

        let session = sqlx::query_as::<_, UserSessionRow>(&query)
            .fetch_optional(self)
            .await?;

        Ok(session)
    }
    async fn session_find_by_userid(&self, userid: uuid::Uuid) -> DBResult<Vec<UserSessionRow>> {
        tracing::debug!(%userid, "looking up sessions for user");

        let query = Query::select()
            .column(Asterisk)
            .from(UserSessions::Table)
            .and_where(Expr::col(UserSessions::UserId).eq(userid))
            .to_string(PostgresQueryBuilder);

        let session = sqlx::query_as::<_, UserSessionRow>(&query)
            .fetch_all(self)
            .await?;

        Ok(session)
    }

    async fn session_revoke(&self, session_id: uuid::Uuid, reason: &str) -> DBResult<()> {
        tracing::debug!(%session_id, "revoking session");

        let query = Query::update()
            .table(UserSessions::Table)
            .values([
                (
                    UserSessions::Status,
                    SessionStatus::Revoked.to_string().into(),
                ),
                (UserSessions::RevokedAt, Expr::current_timestamp().into()),
                (UserSessions::RevokedReason, reason.into()),
            ])
            .and_where(Expr::col(UserSessions::SessionId).eq(session_id))
            .to_string(PostgresQueryBuilder);

        sqlx::query(&query).execute(self).await?;

        Ok(())
    }
}
