pub mod auth;

use axum::Router;

use crate::state::AppState;

/// Build the v1 API router combining all sub-routers.
pub fn router() -> Router<AppState> {
    Router::new().nest("/api/v1", auth::router())
}
