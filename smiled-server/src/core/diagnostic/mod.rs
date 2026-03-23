pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_diagnostic_handler, get_diagnostic_handler, list_diagnostics_handler,
};

/// Build the diagnostic sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/patients/:id/diagnostics",
            get(list_diagnostics_handler).post(create_diagnostic_handler),
        )
        .route(
            "/patients/:id/diagnostics/:diag_id",
            get(get_diagnostic_handler),
        )
}
