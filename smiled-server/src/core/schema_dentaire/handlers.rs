use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::{
    auth::{
        middleware::AuthUser,
        permissions::{PermissionDenied, RequirePermission},
    },
    core::patient::types::ApiResponse,
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
};

use super::{
    queries::{
        bulk_update_paro, get_all_paro_for_schema, get_atm, get_faces_for_teeth, get_occlusion,
        get_paro_global, get_paro_sites_for_teeth, get_schema, get_teeth, get_tooth_by_fdi,
        get_tooth_timeline, init_schema_teeth, insert_atm, insert_occlusion, insert_paro_global,
        insert_schema, list_schema_versions, patient_exists, update_atm, update_occlusion,
        update_paro_global, update_tooth,
    },
    types::{
        CreateSchemaInput, FullSchemaResponse, SchemaVersionQuery, ToothEntry, UpdateAtmInput,
        UpdateOcclusionInput, UpdateParoGlobalInput, UpdateParoInput, UpdateToothInput,
    },
};

// ─── POST /patients/:id/schema ────────────────────────────────────────────────

/// Create a new versioned dental schema for a patient.
///
/// Initializes 32 teeth (permanent), 6 faces + 6 paro sites per tooth,
/// plus empty occlusion, ATM, and paro_global rows.
pub async fn create_schema_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<CreateSchemaInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("schema.write").check(&state, &auth_user).await?;

    if !["permanente", "lacteale", "mixte"].contains(&body.dentition.as_str()) {
        return Err(SchemaApiError::Validation(
            format!("Dentition invalide: '{}'. Valeurs acceptées: permanente, lacteale, mixte", body.dentition),
        ));
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    if !patient_exists(&mut tx, patient_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
    {
        return Err(SchemaApiError::NotFound("Patient introuvable".to_string()));
    }

    let schema = insert_schema(&mut tx, patient_id, auth_user.cabinet_id, auth_user.user_id, &body.dentition)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("23505") || msg.contains("unique") {
                SchemaApiError::Conflict("Une version de schéma est déjà en cours de création".to_string())
            } else {
                SchemaApiError::Database(msg)
            }
        })?;

    init_schema_teeth(&mut tx, schema.id, &schema.dentition)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    insert_occlusion(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    insert_atm(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    insert_paro_global(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let full = load_full_schema(&mut tx, schema)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(full))))
}

// ─── GET /patients/:id/schema ─────────────────────────────────────────────────

/// Return the full dental schema (latest or specific version).
pub async fn get_schema_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Query(params): Query<SchemaVersionQuery>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, params.version)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let full = load_full_schema(&mut tx, schema)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(full)))
}

// ─── GET /patients/:id/schema/versions ───────────────────────────────────────

/// List all schema versions for a patient.
pub async fn list_schema_versions_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let versions = list_schema_versions(&mut tx, patient_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(versions)))
}

// ─── GET /patients/:id/schema/dent/:fdi ──────────────────────────────────────

/// Get a single tooth by FDI number (from the latest schema).
pub async fn get_tooth_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, fdi)): Path<(Uuid, i16)>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let tooth = get_tooth_by_fdi(&mut tx, schema.id, fdi)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound(format!("Dent {fdi} introuvable")))?;

    let tooth_id = tooth.id;
    let faces = get_faces_for_teeth(&mut tx, &[tooth_id])
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let paro_sites = get_paro_sites_for_teeth(&mut tx, &[tooth_id])
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let entry = ToothEntry { tooth, faces, paro_sites };
    Ok(Json(ApiResponse::success(entry)))
}

// ─── PUT /patients/:id/schema/dent/:fdi ──────────────────────────────────────

/// Update a single tooth's clinical state by FDI number.
pub async fn update_tooth_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, fdi)): Path<(Uuid, i16)>,
    Json(body): Json<UpdateToothInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("schema.write").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let tooth = get_tooth_by_fdi(&mut tx, schema.id, fdi)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound(format!("Dent {fdi} introuvable")))?;

    let updated = update_tooth(&mut tx, tooth.id, &body)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound(format!("Dent {fdi} introuvable")))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(updated)))
}

// ─── GET /patients/:id/schema/paro ───────────────────────────────────────────

/// Get all paro sites for the current schema.
pub async fn get_paro_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let sites = get_all_paro_for_schema(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(sites)))
}

// ─── PUT /patients/:id/schema/paro ───────────────────────────────────────────

/// Bulk update paro site measurements for the current schema.
pub async fn update_paro_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<UpdateParoInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("paro.write").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let updated = bulk_update_paro(&mut tx, schema.id, &body.sites)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(updated)))
}

// ─── GET /patients/:id/schema/occlusion ──────────────────────────────────────

/// Get occlusion data for the current schema.
pub async fn get_occlusion_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let occlusion = get_occlusion(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(occlusion)))
}

// ─── PUT /patients/:id/schema/occlusion ──────────────────────────────────────

/// Update occlusion data for the current schema.
pub async fn update_occlusion_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<UpdateOcclusionInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("schema.write").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let updated = update_occlusion(&mut tx, schema.id, &body)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Occlusion introuvable".to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(updated)))
}

// ─── GET /patients/:id/schema/atm ────────────────────────────────────────────

/// Get ATM data for the current schema.
pub async fn get_atm_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let atm = get_atm(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(atm)))
}

// ─── PUT /patients/:id/schema/atm ────────────────────────────────────────────

/// Update ATM data for the current schema.
pub async fn update_atm_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<UpdateAtmInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("schema.write").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let updated = update_atm(&mut tx, schema.id, &body)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("ATM introuvable".to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(updated)))
}

// ─── GET /patients/:id/schema/paro-global ────────────────────────────────────

/// Get paro global classification for the current schema.
pub async fn get_paro_global_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let paro_global = get_paro_global(&mut tx, schema.id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(paro_global)))
}

// ─── PUT /patients/:id/schema/paro-global ────────────────────────────────────

/// Update paro global classification for the current schema.
pub async fn update_paro_global_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<UpdateParoGlobalInput>,
) -> Result<impl IntoResponse, SchemaApiError> {
    RequirePermission("paro.write").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let schema = get_schema(&mut tx, patient_id, None)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Schéma dentaire introuvable".to_string()))?;

    let updated = update_paro_global(&mut tx, schema.id, &body)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?
        .ok_or_else(|| SchemaApiError::NotFound("Paro global introuvable".to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(updated)))
}

// ─── GET /patients/:id/dent/:fdi/timeline ────────────────────────────────────

/// Get the clinical event timeline for a specific tooth.
pub async fn get_tooth_timeline_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, fdi)): Path<(Uuid, i16)>,
) -> Result<impl IntoResponse, SchemaApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    let events = get_tooth_timeline(&mut tx, patient_id, fdi)
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| SchemaApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(events)))
}

// ─── Private Helpers ──────────────────────────────────────────────────────────

/// Load a full schema response (teeth + faces + paro + occlusion + ATM + paro_global).
async fn load_full_schema(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema: crate::core::schema_dentaire::types::DentalSchema,
) -> Result<FullSchemaResponse, sqlx::Error> {
    let teeth = get_teeth(tx, schema.id).await?;
    let tooth_ids: Vec<Uuid> = teeth.iter().map(|t| t.id).collect();
    let all_faces = get_faces_for_teeth(tx, &tooth_ids).await?;
    let all_paro = get_paro_sites_for_teeth(tx, &tooth_ids).await?;
    let occlusion = get_occlusion(tx, schema.id).await?;
    let atm = get_atm(tx, schema.id).await?;
    let paro_global = get_paro_global(tx, schema.id).await?;

    let dents = teeth
        .into_iter()
        .map(|tooth| {
            let faces = all_faces
                .iter()
                .filter(|f| f.dent_id == tooth.id)
                .cloned()
                .collect();
            let paro_sites = all_paro
                .iter()
                .filter(|p| p.dent_id == tooth.id)
                .cloned()
                .collect();
            ToothEntry { tooth, faces, paro_sites }
        })
        .collect();

    Ok(FullSchemaResponse {
        schema,
        dents,
        occlusion,
        atm,
        paro_global,
    })
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SchemaApiError {
    NotFound(String),
    Forbidden,
    Validation(String),
    Conflict(String),
    Database(String),
}

impl From<PermissionDenied> for SchemaApiError {
    fn from(e: PermissionDenied) -> Self {
        match e {
            PermissionDenied::Forbidden => Self::Forbidden,
            PermissionDenied::Internal(msg) => Self::Database(msg),
        }
    }
}

impl IntoResponse for SchemaApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            SchemaApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            SchemaApiError::Forbidden => (StatusCode::FORBIDDEN, "Action non autorisée".to_string()),
            SchemaApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            SchemaApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            SchemaApiError::Database(e) => {
                tracing::error!("Database error in schema handler: {e}");
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
