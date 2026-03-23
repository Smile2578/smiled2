use sqlx::PgPool;

use crate::config::Config;

/// Shared application state passed to all handlers via Axum's `State` extractor.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}

impl AppState {
    pub fn new(pool: PgPool, config: Config) -> Self {
        Self { pool, config }
    }
}
