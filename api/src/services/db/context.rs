use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TenantContext {
    async fn with_tenant_context<F, Fut, T>(pool: &PgPool, org_id: &Uuid, f: F) -> sqlx::Result<T>
    where
        F: FnOnce(Transaction<'static, Postgres>) -> Fut + Send + 'static,
        Fut: Future<Output = sqlx::Result<(T, Transaction<'static, Postgres>)>> + Send,
        T: Send + 'static;
}

#[async_trait::async_trait]
impl TenantContext for PgPool {
    async fn with_tenant_context<F, Fut, T>(pool: &PgPool, org_id: &Uuid, f: F) -> sqlx::Result<T>
    where
        F: FnOnce(Transaction<'static, Postgres>) -> Fut + Send + 'static,
        Fut: Future<Output = sqlx::Result<(T, Transaction<'static, Postgres>)>> + Send,
        T: Send + 'static,
    {
        let mut tx = pool.begin().await?;

        let query = format!("SELECT set_config('app.organization', '{}', true);", org_id);
        sqlx::query(&query).execute(&mut *tx).await?;

        let (result, tx) = f(tx).await?;

        tx.commit().await?;

        Ok(result)
    }
}
