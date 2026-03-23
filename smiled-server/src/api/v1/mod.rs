pub mod auth;

use axum::Router;

use crate::{core::patient, state::AppState};

/// Build the v1 API router combining all sub-routers.
pub fn router() -> Router<AppState> {
    Router::new().nest("/api/v1", auth::router().merge(patient::router()))
}
