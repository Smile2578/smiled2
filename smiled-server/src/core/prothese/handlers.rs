use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{middleware::AuthUser, permissions::RequirePermission},
    error::ApiError,
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
    types::ApiResponse,
};

use super::{
    queries::{insert_prothese, list_protheses, soft_delete_prothese, update_prothese},
    types::{CreateProtheseAmovible, UpdateProtheseAmovible, VALID_ARCADES},
};

// ─── GET /patients/:patient_id/protheses-amovibles ──────────────────────────

/// List all protheses amovibles for a patient.
pub async fn list_protheses_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let protheses = list_protheses(&mut tx, patient_id).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(protheses)))
}

// ─── POST /patients/:patient_id/protheses-amovibles ─────────────────────────

/// Create a new prothese amovible for a patient.
pub async fn create_prothese_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<CreateProtheseAmovible>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("patient.write_clinical").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    if !VALID_ARCADES.contains(&body.arcade.as_str()) {
        return Err(ApiError::Validation(format!(
            "Arcade invalide: '{}'. Valeurs acceptées: {}",
            body.arcade,
            VALID_ARCADES.join(", ")
        )));
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let prothese = insert_prothese(&mut tx, patient_id, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("23503") {
                ApiError::NotFound
            } else {
                ApiError::Database(e)
            }
        })?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(prothese))))
}

// ─── PUT /patients/:patient_id/protheses-amovibles/:pa_id ───────────────────

/// Update a prothese amovible.
pub async fn update_prothese_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, pa_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdateProtheseAmovible>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("patient.write_clinical").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    if let Some(ref arcade) = body.arcade {
        if !VALID_ARCADES.contains(&arcade.as_str()) {
            return Err(ApiError::Validation(format!(
                "Arcade invalide: '{}'. Valeurs acceptées: {}",
                arcade,
                VALID_ARCADES.join(", ")
            )));
        }
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let prothese = update_prothese(&mut tx, patient_id, pa_id, &body)
        .await?
        .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(prothese)))
}

// ─── DELETE /patients/:patient_id/protheses-amovibles/:pa_id ────────────────

/// Soft-delete a prothese amovible.
pub async fn delete_prothese_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, pa_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("patient.write_clinical").check(&state, &auth_user).await?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let deleted = soft_delete_prothese(&mut tx, patient_id, pa_id).await?;

    if !deleted {
        return Err(ApiError::NotFound);
    }

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
