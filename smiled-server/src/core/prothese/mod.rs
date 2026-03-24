pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_prothese_handler, delete_prothese_handler, list_protheses_handler,
    update_prothese_handler,
};

/// Build the prothese amovible sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/patients/:id/protheses-amovibles",
            get(list_protheses_handler).post(create_prothese_handler),
        )
        .route(
            "/patients/:id/protheses-amovibles/:pa_id",
            axum::routing::put(update_prothese_handler).delete(delete_prothese_handler),
        )
}
