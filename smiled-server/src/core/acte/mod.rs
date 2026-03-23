pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_acte_handler, list_actes_handler, override_tarif_handler, toggle_acte_handler,
    update_acte_handler,
};

/// Build the acte sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/actes", get(list_actes_handler).post(create_acte_handler))
        .route("/actes/:id", axum::routing::put(update_acte_handler))
        .route("/actes/:id/toggle", axum::routing::put(toggle_acte_handler))
        .route(
            "/actes/:id/tarif",
            axum::routing::put(override_tarif_handler),
        )
}
