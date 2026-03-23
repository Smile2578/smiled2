pub mod handlers;
pub mod queries;
pub mod questionnaire;
pub mod types;

use axum::{routing::get, Router};

use crate::state::AppState;
use handlers::{
    create_patient_handler, delete_patient_handler, get_patient_handler,
    get_questionnaire_handler, list_patients_handler, update_patient_handler,
    upsert_questionnaire_handler,
};

/// Build the patient sub-router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/patients", get(list_patients_handler).post(create_patient_handler))
        .route(
            "/patients/:id",
            get(get_patient_handler)
                .put(update_patient_handler)
                .delete(delete_patient_handler),
        )
        .route(
            "/patients/:id/questionnaire",
            get(get_questionnaire_handler).put(upsert_questionnaire_handler),
        )
}
