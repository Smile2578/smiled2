pub mod auth;

use axum::{routing::get, Router};

use crate::{
    audit::handlers::list_audit_log_handler,
    core::{acte, cabinet, diagnostic, document, materiau, patient, pdt, prothese, schema_dentaire},
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
            .merge(schema_dentaire::router())
            .merge(diagnostic::router())
            .merge(pdt::router())
            .merge(document::router())
            .merge(cabinet::router())
            .merge(prothese::router())
            .route("/audit-log", get(list_audit_log_handler)),
    )
}
