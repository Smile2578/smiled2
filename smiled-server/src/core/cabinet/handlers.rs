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
    queries::{
        get_cabinet, insert_cabinet_user, list_cabinet_users, update_cabinet, update_cabinet_user,
    },
    types::{InviteUser, UpdateCabinet, UpdateUser, VALID_ROLES},
};

// ─── GET /api/v1/cabinet ────────────────────────────────────────────────────

/// Return cabinet info for the authenticated user's cabinet.
pub async fn get_cabinet_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let cabinet = get_cabinet(&mut tx, auth_user.cabinet_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(cabinet)))
}

// ─── PUT /api/v1/cabinet ────────────────────────────────────────────────────

/// Update cabinet settings (nom, adresse, siret, finess).
pub async fn update_cabinet_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<UpdateCabinet>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("settings.cabinet").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let cabinet = update_cabinet(
        &mut tx,
        auth_user.cabinet_id,
        body.nom.as_deref(),
        body.adresse.as_deref(),
        body.siret.as_deref(),
        body.finess.as_deref(),
    )
    .await?
    .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(cabinet)))
}

// ─── GET /api/v1/cabinet/users ──────────────────────────────────────────────

/// List all users in the authenticated user's cabinet.
pub async fn list_cabinet_users_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let users = list_cabinet_users(&mut tx, auth_user.cabinet_id).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(users)))
}

// ─── POST /api/v1/cabinet/users/invite ──────────────────────────────────────

/// Invite (create) a new user in the cabinet.
pub async fn invite_user_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<InviteUser>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("settings.users").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    if !VALID_ROLES.contains(&body.role.as_str()) {
        return Err(ApiError::Validation(format!(
            "Rôle invalide: '{}'. Valeurs acceptées: {}",
            body.role,
            VALID_ROLES.join(", ")
        )));
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let user = insert_cabinet_user(
        &mut tx,
        auth_user.cabinet_id,
        &body.nom,
        &body.prenom,
        &body.email,
        &body.role,
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") {
            ApiError::Conflict("Un utilisateur avec cet email existe déjà".to_string())
        } else {
            ApiError::Database(e)
        }
    })?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(user))))
}

// ─── PUT /api/v1/cabinet/users/:id ──────────────────────────────────────────

/// Update a user's role and/or actif status.
pub async fn update_user_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UpdateUser>,
) -> Result<impl IntoResponse, ApiError> {
    RequirePermission("settings.users").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    if let Some(ref role) = body.role {
        if !VALID_ROLES.contains(&role.as_str()) {
            return Err(ApiError::Validation(format!(
                "Rôle invalide: '{}'. Valeurs acceptées: {}",
                role,
                VALID_ROLES.join(", ")
            )));
        }
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let user = update_cabinet_user(
        &mut tx,
        auth_user.cabinet_id,
        user_id,
        body.role.as_deref(),
        body.actif,
    )
    .await?
    .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    Ok(Json(ApiResponse::success(user)))
}
