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
    queries::{insert_materiau, list_categories, list_materiaux, update_materiau},
    types::{CreateMateriau, MateriauListParams, UpdateMateriau},
};
use crate::core::patient::types::ApiResponse;

// ─── GET /api/v1/materiaux ────────────────────────────────────────────────────

/// List all matériaux (système + cabinet) with category info.
pub async fn list_materiaux_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<MateriauListParams>,
) -> Result<impl IntoResponse, MateriauApiError> {
    let materiaux = list_materiaux(&state.pool, auth_user.cabinet_id, &params)
        .await
        .map_err(|e| MateriauApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(materiaux)))
}

// ─── GET /api/v1/materiaux/categories ─────────────────────────────────────────

/// List all matériau categories.
pub async fn list_categories_handler(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, MateriauApiError> {
    let categories = list_categories(&state.pool)
        .await
        .map_err(|e| MateriauApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(categories)))
}

// ─── POST /api/v1/materiaux ───────────────────────────────────────────────────

/// Create a cabinet-specific matériau.
pub async fn create_materiau_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreateMateriau>,
) -> Result<impl IntoResponse, MateriauApiError> {
    body.validate()
        .map_err(|e| MateriauApiError::Validation(e.to_string()))?;

    let materiau = insert_materiau(&state.pool, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| MateriauApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(materiau))))
}

// ─── PUT /api/v1/materiaux/:id ────────────────────────────────────────────────

/// Update a cabinet-level matériau.
pub async fn update_materiau_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateMateriau>,
) -> Result<impl IntoResponse, MateriauApiError> {
    body.validate()
        .map_err(|e| MateriauApiError::Validation(e.to_string()))?;

    let materiau = update_materiau(&state.pool, id, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| MateriauApiError::Database(e.to_string()))?
        .ok_or(MateriauApiError::NotFound)?;

    Ok(Json(ApiResponse::success(materiau)))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum MateriauApiError {
    NotFound,
    Validation(String),
    Database(String),
}

impl IntoResponse for MateriauApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            MateriauApiError::NotFound => {
                (StatusCode::NOT_FOUND, "Matériau introuvable".to_string())
            }
            MateriauApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            MateriauApiError::Database(e) => {
                tracing::error!("Database error in materiau handler: {e}");
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
