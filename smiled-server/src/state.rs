use std::time::Duration;

use moka::future::Cache;
use sqlx::PgPool;

use crate::{config::Config, core::document::storage::S3Storage};

/// Shared application state passed to all handlers via Axum's `State` extractor.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    /// S3-compatible storage client. `None` when S3 is not available (e.g., test environments).
    pub s3: Option<S3Storage>,
    /// Per-user permission cache. Key: `"{user_id}:{permission_code}"`, value: allowed.
    pub permission_cache: Cache<String, bool>,
}

fn new_permission_cache() -> Cache<String, bool> {
    Cache::builder()
        .max_capacity(10_000)
        .time_to_live(Duration::from_secs(300))
        .build()
}

impl AppState {
    pub fn new(pool: PgPool, config: Config) -> Self {
        Self {
            pool,
            config,
            s3: None,
            permission_cache: new_permission_cache(),
        }
    }

    pub fn with_s3(pool: PgPool, config: Config, s3: S3Storage) -> Self {
        Self {
            pool,
            config,
            s3: Some(s3),
            permission_cache: new_permission_cache(),
        }
    }
}
