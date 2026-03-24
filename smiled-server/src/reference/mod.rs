pub mod motif;
pub mod teinte;
pub mod trouble;

use axum::{routing::get, Router};

use crate::state::AppState;
use motif::{create_motif_handler, delete_motif_handler, list_motifs_handler, update_motif_handler};
use teinte::{create_teinte_handler, list_teintes_handler, update_teinte_handler};
use trouble::list_troubles_handler;

/// Build the reference data sub-router (teintes, motifs, troubles).
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/teintes",
            get(list_teintes_handler).post(create_teinte_handler),
        )
        .route(
            "/teintes/:id",
            axum::routing::put(update_teinte_handler),
        )
        .route(
            "/motifs",
            get(list_motifs_handler).post(create_motif_handler),
        )
        .route(
            "/motifs/:id",
            axum::routing::put(update_motif_handler).delete(delete_motif_handler),
        )
        .route("/troubles", get(list_troubles_handler))
}
