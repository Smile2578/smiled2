pub mod auth;

use axum::Router;

use crate::{
    core::{acte, materiau, patient, schema_dentaire},
    reference,
    state::AppState,
};

/// Build the v1 API router combining all sub-routers.
pub fn router() -> Router<AppState> {
    Router::new().nest(
        "/api/v1",
        auth::router()
            .merge(patient::router())
            .merge(acte::router())
            .merge(materiau::router())
            .merge(reference::router())
            .merge(schema_dentaire::router()),
    )
}
