use sqlx::PgPool;
use uuid::Uuid;

/// Begin a transaction and set the tenant context within it.
///
/// The returned transaction has the `app.current_tenant` GUC set, so all
/// queries run through it will respect RLS automatically.
///
/// Uses `set_config()` with parameterized binding (no string concatenation).
/// The third argument `true` makes it transaction-scoped (equivalent to SET LOCAL).
pub async fn begin_tenant_transaction(
    pool: &PgPool,
    cabinet_id: Uuid,
) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(cabinet_id.to_string())
        .execute(&mut *tx)
        .await?;

    Ok(tx)
}
