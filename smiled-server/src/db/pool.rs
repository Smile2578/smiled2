use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// Create a PostgreSQL connection pool.
///
/// Configured for production use:
/// - min 2 warm connections
/// - max 20 connections
/// - 5 s acquire timeout (fail fast)
/// - 10 min idle timeout
/// - 30 min max lifetime (recycle connections)
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .min_connections(2)
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
}
