pub mod handlers;
pub mod queries;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_schema_handler, get_atm_handler, get_occlusion_handler, get_paro_global_handler,
    get_paro_handler, get_schema_handler, get_tooth_handler, get_tooth_timeline_handler,
    list_schema_versions_handler, update_atm_handler, update_occlusion_handler,
    update_paro_global_handler, update_paro_handler, update_tooth_handler,
};

/// Build the schema dentaire sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Schema lifecycle
        .route(
            "/patients/:id/schema",
            get(get_schema_handler).post(create_schema_handler),
        )
        .route(
            "/patients/:id/schema/versions",
            get(list_schema_versions_handler),
        )
        // Single tooth
        .route(
            "/patients/:id/schema/dent/:fdi",
            get(get_tooth_handler).put(update_tooth_handler),
        )
        // Paro bulk
        .route(
            "/patients/:id/schema/paro",
            get(get_paro_handler).put(update_paro_handler),
        )
        // Occlusion
        .route(
            "/patients/:id/schema/occlusion",
            get(get_occlusion_handler).put(update_occlusion_handler),
        )
        // ATM
        .route(
            "/patients/:id/schema/atm",
            get(get_atm_handler).put(update_atm_handler),
        )
        // Paro global
        .route(
            "/patients/:id/schema/paro-global",
            get(get_paro_global_handler).put(update_paro_global_handler),
        )
        // Tooth timeline (historique)
        .route(
            "/patients/:id/dent/:fdi/timeline",
            get(get_tooth_timeline_handler),
        )
}
