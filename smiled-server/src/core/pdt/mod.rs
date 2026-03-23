pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_pdt_handler, generate_pdf_handler, list_pdts_handler, update_pdt_handler,
};

/// Build the PDT (plan de traitement) sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/patients/:id/pdts",
            get(list_pdts_handler).post(create_pdt_handler),
        )
        .route(
            "/patients/:id/pdts/:pdt_id",
            axum::routing::put(update_pdt_handler),
        )
        .route(
            "/patients/:id/pdts/:pdt_id/pdf",
            axum::routing::post(generate_pdf_handler),
        )
}
