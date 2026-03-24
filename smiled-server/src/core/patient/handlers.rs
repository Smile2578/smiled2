use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{
        middleware::AuthUser,
        permissions::{PermissionDenied, RequirePermission},
    },
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
};

use super::{
    queries::{get_patient_by_id, insert_patient, list_patients, soft_delete_patient, update_patient},
    questionnaire::{get_questionnaire, upsert_questionnaire},
    types::{
        ApiResponse, CreatePatient, PaginationMeta, PatientListParams, QuestionnaireInput,
        UpdatePatient,
    },
};

// ─── GET /api/v1/patients ─────────────────────────────────────────────────────

/// List patients with optional search and pagination.
pub async fn list_patients_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PatientListParams>,
) -> Result<impl IntoResponse, PatientApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    let limit = params.limit.clamp(1, 100);
    let page = params.page.max(1);

    let (patients, total) = list_patients(
        &mut tx,
        params.search.as_deref(),
        page,
        limit,
    )
    .await
    .map_err(|e| PatientApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success_with_meta(
        patients,
        PaginationMeta { total, page, limit },
    )))
}

// ─── POST /api/v1/patients ────────────────────────────────────────────────────

/// Create a new patient.
pub async fn create_patient_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(body): Json<CreatePatient>,
) -> Result<impl IntoResponse, PatientApiError> {
    RequirePermission("patient.write_admin").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| PatientApiError::Validation(e.to_string()))?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    let patient = insert_patient(&mut tx, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(patient))))
}

// ─── GET /api/v1/patients/:id ─────────────────────────────────────────────────

/// Get a single patient by ID.
pub async fn get_patient_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, PatientApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    let patient = get_patient_by_id(&mut tx, id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?
        .ok_or(PatientApiError::NotFound)?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(patient)))
}

// ─── PUT /api/v1/patients/:id ─────────────────────────────────────────────────

/// Partially update a patient record.
pub async fn update_patient_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdatePatient>,
) -> Result<impl IntoResponse, PatientApiError> {
    RequirePermission("patient.write_admin").check(&state, &auth_user).await?;
    body.validate()
        .map_err(|e| PatientApiError::Validation(e.to_string()))?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    let patient = update_patient(&mut tx, id, &body)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?
        .ok_or(PatientApiError::NotFound)?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(patient)))
}

// ─── DELETE /api/v1/patients/:id ──────────────────────────────────────────────

/// Soft-delete a patient.
pub async fn delete_patient_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, PatientApiError> {
    RequirePermission("patient.write_admin").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    let deleted = soft_delete_patient(&mut tx, id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    if !deleted {
        return Err(PatientApiError::NotFound);
    }

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// ─── GET /api/v1/patients/:id/questionnaire ───────────────────────────────────

/// Get the latest medical questionnaire for a patient.
pub async fn get_questionnaire_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, PatientApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    // Verify patient exists and belongs to this cabinet (RLS handles the latter)
    let patient_exists = get_patient_by_id(&mut tx, id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    if patient_exists.is_none() {
        return Err(PatientApiError::NotFound);
    }

    let questionnaire = get_questionnaire(&mut tx, id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    match questionnaire {
        Some(q) => Ok((StatusCode::OK, Json(ApiResponse::success(q))).into_response()),
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "Aucun questionnaire médical trouvé pour ce patient".to_string(),
            )),
        )
            .into_response()),
    }
}

// ─── PUT /api/v1/patients/:id/questionnaire ───────────────────────────────────

/// Create or update the medical questionnaire for a patient.
pub async fn upsert_questionnaire_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(body): Json<QuestionnaireInput>,
) -> Result<impl IntoResponse, PatientApiError> {
    RequirePermission("patient.write_clinical").check(&state, &auth_user).await?;
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    // Verify patient exists
    let patient_exists = get_patient_by_id(&mut tx, id)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    if patient_exists.is_none() {
        return Err(PatientApiError::NotFound);
    }

    let questionnaire = upsert_questionnaire(&mut tx, id, auth_user.cabinet_id, &body)
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PatientApiError::Database(e.to_string()))?;

    Ok((StatusCode::OK, Json(ApiResponse::success(questionnaire))))
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PatientApiError {
    NotFound,
    Forbidden,
    Validation(String),
    Database(String),
}

impl From<PermissionDenied> for PatientApiError {
    fn from(e: PermissionDenied) -> Self {
        match e {
            PermissionDenied::Forbidden => Self::Forbidden,
            PermissionDenied::Internal(msg) => Self::Database(msg),
        }
    }
}

impl IntoResponse for PatientApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PatientApiError::NotFound => {
                (StatusCode::NOT_FOUND, "Patient introuvable".to_string())
            }
            PatientApiError::Forbidden => {
                (StatusCode::FORBIDDEN, "Action non autorisée".to_string())
            }
            PatientApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            PatientApiError::Database(e) => {
                tracing::error!("Database error in patient handler: {e}");
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
