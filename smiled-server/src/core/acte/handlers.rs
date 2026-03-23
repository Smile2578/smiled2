use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{auth::middleware::AuthUser, state::AppState};

use super::{
    queries::{acte_exists, insert_acte, list_actes, toggle_acte, update_acte, upsert_tarif},
    types::{ActeListParams, CreateActe, TarifOverride, UpdateActe},
};
use crate::core::patient::types::ApiResponse;

// ─── GET /api/v1/actes ────────────────────────────────────────────────────────

/// List all actes (système + cabinet) with optional tarif override.
pub async fn list_actes_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<ActeListParams>,
) -> Result<impl IntoResponse, ActeApiError> {
    let actes = list_actes(&state.pool, auth_user.cabinet_id, &params)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(actes)))
}

// ─── POST /api/v1/actes ───────────────────────────────────────────────────────

/// Create a cabinet-specific acte.
pub async fn create_acte_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreateActe>,
) -> Result<impl IntoResponse, ActeApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ActeApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| ActeApiError::Validation(e.to_string()))?;

    let acte = insert_acte(&state.pool, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(acte))))
}

// ─── PUT /api/v1/actes/:id ────────────────────────────────────────────────────

/// Update a cabinet-level acte (system actes are read-only).
pub async fn update_acte_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateActe>,
) -> Result<impl IntoResponse, ActeApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ActeApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| ActeApiError::Validation(e.to_string()))?;

    let acte = update_acte(&state.pool, id, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?
        .ok_or(ActeApiError::NotFound)?;

    Ok(Json(ApiResponse::success(acte)))
}

// ─── PUT /api/v1/actes/:id/toggle ────────────────────────────────────────────

/// Toggle the actif flag on a cabinet-level acte.
pub async fn toggle_acte_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ActeApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ActeApiError::Forbidden);
    }
    let acte = toggle_acte(&state.pool, id, auth_user.cabinet_id)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?
        .ok_or(ActeApiError::NotFound)?;

    Ok(Json(ApiResponse::success(acte)))
}

// ─── PUT /api/v1/actes/:id/tarif ─────────────────────────────────────────────

/// Override or set the tarif for any acte within this cabinet.
pub async fn override_tarif_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<TarifOverride>,
) -> Result<impl IntoResponse, ActeApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ActeApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| ActeApiError::Validation(e.to_string()))?;
    let exists = acte_exists(&state.pool, id, auth_user.cabinet_id)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?;

    if !exists {
        return Err(ActeApiError::NotFound);
    }

    upsert_tarif(&state.pool, id, auth_user.cabinet_id, body.prix)
        .await
        .map_err(|e| ActeApiError::Database(e.to_string()))?;

    Ok(Json(
        serde_json::json!({ "success": true, "message": "Tarif mis à jour" }),
    ))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum ActeApiError {
    NotFound,
    Forbidden,
    Validation(String),
    Database(String),
}

impl IntoResponse for ActeApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ActeApiError::NotFound => (StatusCode::NOT_FOUND, "Acte introuvable".to_string()),
            ActeApiError::Forbidden => (StatusCode::FORBIDDEN, "Action non autorisée".to_string()),
            ActeApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ActeApiError::Database(e) => {
                tracing::error!("Database error in acte handler: {e}");
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
