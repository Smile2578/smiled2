use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    error::ApiError,
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
    types::{ApiResponse, PaginationMeta},
};

#[derive(Debug, Deserialize)]
pub struct AuditLogParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub changes: Option<serde_json::Value>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status_code: Option<i32>,
    pub request_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

/// GET /api/v1/audit-log -- paginated audit log for the current cabinet.
/// Restricted to `titulaire` and `admin` roles.
pub async fn list_audit_log_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<AuditLogParams>,
) -> Result<Json<ApiResponse<Vec<AuditLogEntry>>>, ApiError> {
    if !matches!(auth_user.role.as_str(), "titulaire" | "admin") {
        return Err(ApiError::Forbidden);
    }

    let limit = params.limit.unwrap_or(50).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id).await?;

    let total: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM audit_log")
        .fetch_one(&mut *tx)
        .await?;

    let rows = sqlx::query(
        "SELECT id, user_id, action, entity_type, entity_id, changes,
                method, path, status_code, request_id, created_at
         FROM audit_log
         ORDER BY created_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    let entries: Vec<AuditLogEntry> = rows
        .iter()
        .map(|row| AuditLogEntry {
            id: row.get("id"),
            user_id: row.get("user_id"),
            action: row.get("action"),
            entity_type: row.get("entity_type"),
            entity_id: row.get("entity_id"),
            changes: row.get("changes"),
            method: row.get("method"),
            path: row.get("path"),
            status_code: row.get("status_code"),
            request_id: row.get("request_id"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(Json(ApiResponse::success_with_meta(
        entries,
        PaginationMeta { total, page, limit },
    )))
}
