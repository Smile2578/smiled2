use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

use crate::{auth::middleware::AuthUser, core::patient::types::ApiResponse, state::AppState};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MotifConsultation {
    pub id: Uuid,
    pub libelle: String,
    pub sous_questions: Option<Value>,
    pub niveau: String,
    pub cabinet_id: Option<Uuid>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMotif {
    #[validate(length(min = 1, max = 300, message = "Le libellé est requis"))]
    pub libelle: String,
    pub sous_questions: Option<Value>,
}

// ─── Queries ──────────────────────────────────────────────────────────────────

pub async fn list_motifs_query(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
) -> Result<Vec<MotifConsultation>, sqlx::Error> {
    sqlx::query_as!(
        MotifConsultation,
        r#"
        SELECT id, libelle, sous_questions, niveau, cabinet_id, actif
        FROM motif_consultation
        WHERE (niveau = 'systeme' OR cabinet_id = $1) AND actif = true
        ORDER BY libelle ASC
        "#,
        cabinet_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_motif_query(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    input: &CreateMotif,
) -> Result<MotifConsultation, sqlx::Error> {
    sqlx::query_as!(
        MotifConsultation,
        r#"
        INSERT INTO motif_consultation (libelle, sous_questions, niveau, cabinet_id)
        VALUES ($1, $2, 'cabinet', $3)
        RETURNING id, libelle, sous_questions, niveau, cabinet_id, actif
        "#,
        input.libelle,
        input.sous_questions,
        cabinet_id,
    )
    .fetch_one(pool)
    .await
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

pub async fn list_motifs_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, MotifApiError> {
    let motifs = list_motifs_query(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| MotifApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(motifs)))
}

pub async fn create_motif_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreateMotif>,
) -> Result<impl IntoResponse, MotifApiError> {
    if !auth_user.can_manage_settings() {
        return Err(MotifApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| MotifApiError::Validation(e.to_string()))?;

    let motif = insert_motif_query(&state.pool, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| MotifApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(motif))))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum MotifApiError {
    Forbidden,
    Validation(String),
    Database(String),
}

impl IntoResponse for MotifApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MotifApiError::Forbidden => (StatusCode::FORBIDDEN, "Action non autorisée".to_string()),
            MotifApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            MotifApiError::Database(e) => {
                tracing::error!("Database error in motif handler: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
        };

        (
            status,
            Json(serde_json::json!({ "success": false, "error": message })),
        )
            .into_response()
    }
}
