pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_materiau_handler, list_categories_handler, list_materiaux_handler,
    update_materiau_handler,
};

/// Build the matériau sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/materiaux",
            get(list_materiaux_handler).post(create_materiau_handler),
        )
        .route(
            "/materiaux/categories",
            get(list_categories_handler),
        )
        .route(
            "/materiaux/:id",
            axum::routing::put(update_materiau_handler),
        )
}
