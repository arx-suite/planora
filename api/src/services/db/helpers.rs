use sqlx::{Pool, Postgres, Transaction};

pub async fn with_org<F, Fut, T>(
    pool: &Pool<Postgres>,
    org_id: &uuid::Uuid,
    f: F,
) -> sqlx::Result<T>
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
