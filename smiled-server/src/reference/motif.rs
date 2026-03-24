use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

use crate::{auth::middleware::AuthUser, error::ApiError, state::AppState, types::ApiResponse};

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

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMotif {
    #[validate(length(min = 1, max = 300, message = "Le libellé est requis"))]
    pub libelle: String,
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

pub async fn update_motif_query(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
    libelle: &str,
) -> Result<Option<MotifConsultation>, sqlx::Error> {
    sqlx::query_as!(
        MotifConsultation,
        r#"
        UPDATE motif_consultation
        SET libelle = $3
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet' AND actif = true
        RETURNING id, libelle, sous_questions, niveau, cabinet_id, actif
        "#,
        id,
        cabinet_id,
        libelle,
    )
    .fetch_optional(pool)
    .await
}

pub async fn soft_delete_motif_query(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE motif_consultation
        SET actif = false
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet' AND actif = true
        "#,
        id,
        cabinet_id,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

pub async fn list_motifs_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, ApiError> {
    let motifs = list_motifs_query(&state.pool, auth_user.cabinet_id).await?;

    Ok(Json(ApiResponse::success(motifs)))
}

pub async fn create_motif_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreateMotif>,
) -> Result<impl IntoResponse, ApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let motif = insert_motif_query(&state.pool, auth_user.cabinet_id, &body).await?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(motif))))
}

pub async fn update_motif_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateMotif>,
) -> Result<impl IntoResponse, ApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ApiError::Forbidden);
    }
    body.validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let motif = update_motif_query(&state.pool, id, auth_user.cabinet_id, &body.libelle)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(ApiResponse::success(motif)))
}

pub async fn delete_motif_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    if !auth_user.can_manage_settings() {
        return Err(ApiError::Forbidden);
    }

    let deleted = soft_delete_motif_query(&state.pool, id, auth_user.cabinet_id).await?;

    if !deleted {
        return Err(ApiError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
