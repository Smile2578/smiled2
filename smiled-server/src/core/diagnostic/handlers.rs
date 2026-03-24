use axum::{
    extract::{Path, State},
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
        get_diagnostic_by_id, get_findings_for_diagnostic, insert_diagnostic, insert_findings,
        list_diagnostics_for_patient,
    },
    types::{CreateDiagnostic, DiagnosticWithFindings},
};

// ─── POST /patients/:id/diagnostics ───────────────────────────────────────────

/// Create a new diagnostic with optional findings for a patient.
pub async fn create_diagnostic_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<CreateDiagnostic>,
) -> Result<impl IntoResponse, DiagnosticApiError> {
    RequirePermission("diagnostic.create").check(&state, &auth_user).await?;

    validate_findings(&body)?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    let diagnostic = insert_diagnostic(
        &mut tx,
        patient_id,
        auth_user.cabinet_id,
        auth_user.user_id,
        &body,
    )
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("23503") {
            DiagnosticApiError::NotFound("Patient introuvable".to_string())
        } else {
            DiagnosticApiError::Database(msg)
        }
    })?;

    let findings = insert_findings(&mut tx, diagnostic.id, &body.findings)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    let response = DiagnosticWithFindings { diagnostic, findings };
    Ok((StatusCode::CREATED, Json(ApiResponse::success(response))))
}

// ─── GET /patients/:id/diagnostics ────────────────────────────────────────────

/// List all diagnostics for a patient (without findings).
pub async fn list_diagnostics_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, DiagnosticApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    let diagnostics = list_diagnostics_for_patient(&mut tx, patient_id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(diagnostics)))
}

// ─── GET /patients/:id/diagnostics/:diag_id ───────────────────────────────────

/// Get a single diagnostic with all its findings.
pub async fn get_diagnostic_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, diagnostic_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, DiagnosticApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    let diagnostic = get_diagnostic_by_id(&mut tx, patient_id, diagnostic_id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?
        .ok_or_else(|| DiagnosticApiError::NotFound("Diagnostic introuvable".to_string()))?;

    let findings = get_findings_for_diagnostic(&mut tx, diagnostic.id)
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| DiagnosticApiError::Database(e.to_string()))?;

    let response = DiagnosticWithFindings { diagnostic, findings };
    Ok(Json(ApiResponse::success(response)))
}

// ─── Validation ───────────────────────────────────────────────────────────────

fn validate_findings(body: &CreateDiagnostic) -> Result<(), DiagnosticApiError> {
    for f in &body.findings {
        if !["praticien", "ia"].contains(&f.source.as_str()) {
            return Err(DiagnosticApiError::Validation(format!(
                "Source invalide: '{}'. Valeurs acceptées: praticien, ia",
                f.source
            )));
        }
        if !(1..=5).contains(&f.severite) {
            return Err(DiagnosticApiError::Validation(
                "Sévérité doit être entre 1 et 5".to_string(),
            ));
        }
    }
    Ok(())
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DiagnosticApiError {
    NotFound(String),
    Forbidden,
    Validation(String),
    Database(String),
}

impl From<PermissionDenied> for DiagnosticApiError {
    fn from(e: PermissionDenied) -> Self {
        match e {
            PermissionDenied::Forbidden => Self::Forbidden,
            PermissionDenied::Internal(msg) => Self::Database(msg),
        }
    }
}

impl IntoResponse for DiagnosticApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DiagnosticApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            DiagnosticApiError::Forbidden => {
                (StatusCode::FORBIDDEN, "Action non autorisée".to_string())
            }
            DiagnosticApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            DiagnosticApiError::Database(e) => {
                tracing::error!("Database error in diagnostic handler: {e}");
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
