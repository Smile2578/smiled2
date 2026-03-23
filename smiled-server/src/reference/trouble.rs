use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{auth::middleware::AuthUser, core::patient::types::ApiResponse, state::AppState};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TroubleMedical {
    pub id: Uuid,
    pub categorie: String,
    pub libelle: String,
    pub icone: Option<String>,
    pub contre_indications: Option<String>,
    pub niveau: String,
    pub cabinet_id: Option<Uuid>,
    pub actif: Option<bool>,
}

// ─── Queries ──────────────────────────────────────────────────────────────────

pub async fn list_troubles_query(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
) -> Result<Vec<TroubleMedical>, sqlx::Error> {
    sqlx::query_as!(
        TroubleMedical,
        r#"
        SELECT id, categorie, libelle, icone, contre_indications, niveau, cabinet_id, actif
        FROM trouble_medical
        WHERE (niveau = 'systeme' OR cabinet_id = $1) AND actif = true
        ORDER BY categorie ASC, libelle ASC
        "#,
        cabinet_id,
    )
    .fetch_all(pool)
    .await
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

pub async fn list_troubles_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, TroubleApiError> {
    let troubles = list_troubles_query(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| TroubleApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(troubles)))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum TroubleApiError {
    Database(String),
}

impl IntoResponse for TroubleApiError {
    fn into_response(self) -> Response {
        let TroubleApiError::Database(e) = self;
        tracing::error!("Database error in trouble handler: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": "Erreur serveur" })),
        )
            .into_response()
    }
}
