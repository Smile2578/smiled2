use sqlx::PgPool;
use uuid::Uuid;

/// Set the RLS tenant context on a database connection.
///
/// Executes `SET LOCAL app.current_tenant = '<uuid>'` which is transaction-scoped
/// and activates the RLS policies defined in migrations 006 and 011.
///
/// # Important
/// This must be called inside an active transaction so that `SET LOCAL` is
/// properly scoped. Outside a transaction `SET LOCAL` behaves like `SET`.
pub async fn set_tenant(
    pool: &PgPool,
    cabinet_id: Uuid,
) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
    let mut conn = pool.acquire().await?;

    sqlx::query(&format!(
        "SET LOCAL app.current_tenant = '{}'",
        cabinet_id
    ))
    .execute(&mut *conn)
    .await?;

    Ok(conn)
}

/// Begin a transaction and set the tenant context within it.
///
/// The returned transaction has the `app.current_tenant` GUC set, so all
/// queries run through it will respect RLS automatically.
pub async fn begin_tenant_transaction(
    pool: &PgPool,
    cabinet_id: Uuid,
) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(&format!(
        "SET LOCAL app.current_tenant = '{}'",
        cabinet_id
    ))
    .execute(&mut *tx)
    .await?;

    Ok(tx)
}
