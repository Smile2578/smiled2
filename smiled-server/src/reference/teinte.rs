use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{auth::middleware::AuthUser, core::patient::types::ApiResponse, state::AppState};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Teinte {
    pub id: Uuid,
    pub systeme: String,
    pub code: String,
    pub libelle: String,
    pub ordre: Option<i32>,
    pub niveau: String,
    pub cabinet_id: Option<Uuid>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTeinte {
    #[validate(length(min = 1, max = 100))]
    pub systeme: String,
    #[validate(length(min = 1, max = 50))]
    pub code: String,
    #[validate(length(min = 1, max = 200))]
    pub libelle: String,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTeinte {
    #[validate(length(min = 1, max = 100))]
    pub systeme: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub libelle: Option<String>,
    pub ordre: Option<i32>,
    pub actif: Option<bool>,
}

// ─── Queries ──────────────────────────────────────────────────────────────────

pub async fn list_teintes_query(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
) -> Result<Vec<Teinte>, sqlx::Error> {
    sqlx::query_as!(
        Teinte,
        r#"
        SELECT id, systeme, code, libelle, ordre, niveau, cabinet_id, actif
        FROM teinte
        WHERE niveau = 'systeme' OR cabinet_id = $1
        ORDER BY systeme ASC, ordre ASC NULLS LAST, code ASC
        "#,
        cabinet_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_teinte_query(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    input: &CreateTeinte,
) -> Result<Teinte, sqlx::Error> {
    sqlx::query_as!(
        Teinte,
        r#"
        INSERT INTO teinte (systeme, code, libelle, ordre, niveau, cabinet_id)
        VALUES ($1, $2, $3, $4, 'cabinet', $5)
        RETURNING id, systeme, code, libelle, ordre, niveau, cabinet_id, actif
        "#,
        input.systeme,
        input.code,
        input.libelle,
        input.ordre,
        cabinet_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn update_teinte_query(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
    input: &UpdateTeinte,
) -> Result<Option<Teinte>, sqlx::Error> {
    sqlx::query_as!(
        Teinte,
        r#"
        UPDATE teinte SET
            systeme    = COALESCE($3, systeme),
            code       = COALESCE($4, code),
            libelle    = COALESCE($5, libelle),
            ordre      = COALESCE($6, ordre),
            actif      = COALESCE($7, actif)
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet'
        RETURNING id, systeme, code, libelle, ordre, niveau, cabinet_id, actif
        "#,
        id,
        cabinet_id,
        input.systeme,
        input.code,
        input.libelle,
        input.ordre,
        input.actif,
    )
    .fetch_optional(pool)
    .await
}

// ─── Handlers ─────────────────────────────────────────────────────────────────

pub async fn list_teintes_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, TeinteApiError> {
    let teintes = list_teintes_query(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| TeinteApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(teintes)))
}

pub async fn create_teinte_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreateTeinte>,
) -> Result<impl IntoResponse, TeinteApiError> {
    body.validate()
        .map_err(|e| TeinteApiError::Validation(e.to_string()))?;

    let teinte = insert_teinte_query(&state.pool, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| TeinteApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(teinte))))
}

pub async fn update_teinte_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTeinte>,
) -> Result<impl IntoResponse, TeinteApiError> {
    body.validate()
        .map_err(|e| TeinteApiError::Validation(e.to_string()))?;

    let teinte = update_teinte_query(&state.pool, id, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| TeinteApiError::Database(e.to_string()))?
        .ok_or(TeinteApiError::NotFound)?;

    Ok(Json(ApiResponse::success(teinte)))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum TeinteApiError {
    NotFound,
    Validation(String),
    Database(String),
}

impl IntoResponse for TeinteApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            TeinteApiError::NotFound => (StatusCode::NOT_FOUND, "Teinte introuvable".to_string()),
            TeinteApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            TeinteApiError::Database(e) => {
                tracing::error!("Database error in teinte handler: {e}");
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
