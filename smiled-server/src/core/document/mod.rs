pub mod handlers;
pub mod queries;
pub mod storage;
pub mod types;

use axum::{routing::post, Router};

use crate::state::AppState;
use handlers::{link_dent_handler, list_documents_handler, upload_document_handler};

/// Build the document sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/patients/:id/documents/upload",
            post(upload_document_handler),
        )
        .route(
            "/patients/:id/documents",
            axum::routing::get(list_documents_handler),
        )
        .route(
            "/patients/:id/documents/:doc_id/link-dent/:fdi",
            post(link_dent_handler),
        )
}
