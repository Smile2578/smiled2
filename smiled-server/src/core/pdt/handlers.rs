use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    core::patient::types::ApiResponse,
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
};

use super::{
    queries::{
        diagnostic_belongs_to_patient, formule_exists_for_diagnostic, get_lignes_for_pdt,
        get_pdt_by_id, insert_lignes, insert_pdt, list_pdts_for_patient, recalculate_prix_total,
        set_pdf_url, update_ligne, update_pdt_fields,
    },
    types::{CreatePdt, PdtWithLines, UpdatePdt},
};

// ─── POST /patients/:id/pdts ───────────────────────────────────────────────────

/// Create a new PDT (formule) for a patient, linked to an existing diagnostic.
///
/// Validates: diagnostic belongs to patient, formule is unique per diagnostic.
/// Inserts all lignes and calculates prix_total.
pub async fn create_pdt_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    Json(body): Json<CreatePdt>,
) -> Result<impl IntoResponse, PdtApiError> {
    if !auth_user.can_write_clinical() {
        return Err(PdtApiError::Forbidden);
    }

    validate_create_input(&body)?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    // 1. Validate diagnostic exists and belongs to this patient
    let belongs = diagnostic_belongs_to_patient(&mut tx, patient_id, body.diagnostic_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;
    if !belongs {
        return Err(PdtApiError::NotFound(
            "Diagnostic introuvable pour ce patient".to_string(),
        ));
    }

    // 2. Validate formule uniqueness per diagnostic
    let exists = formule_exists_for_diagnostic(&mut tx, body.diagnostic_id, &body.formule)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;
    if exists {
        return Err(PdtApiError::Conflict(format!(
            "Un plan de traitement '{}' existe déjà pour ce diagnostic",
            body.formule
        )));
    }

    // 3. Calculate prix_total from lignes
    let prix_total = calculate_prix_total(&body.lignes);

    // 4. Insert PDT
    let pdt = insert_pdt(
        &mut tx,
        patient_id,
        auth_user.cabinet_id,
        auth_user.user_id,
        &body,
        prix_total,
    )
    .await
    .map_err(|e| PdtApiError::Database(e.to_string()))?;

    // 5. Insert all lignes
    let lignes = insert_lignes(&mut tx, pdt.id, &body.lignes)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    let response = PdtWithLines { pdt, lignes };
    Ok((StatusCode::CREATED, Json(ApiResponse::success(response))))
}

// ─── GET /patients/:id/pdts ────────────────────────────────────────────────────

/// List all PDTs for a patient (without lignes).
pub async fn list_pdts_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, PdtApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    let pdts = list_pdts_for_patient(&mut tx, patient_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(pdts)))
}

// ─── PUT /patients/:id/pdts/:pdt_id ───────────────────────────────────────────

/// Update PDT statut and/or lignes statut/prix.
///
/// If lignes are updated, recalculates prix_total.
pub async fn update_pdt_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, pdt_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<UpdatePdt>,
) -> Result<impl IntoResponse, PdtApiError> {
    if !auth_user.can_write_clinical() {
        return Err(PdtApiError::Forbidden);
    }

    validate_update_input(&body)?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    let pdt = update_pdt_fields(&mut tx, pdt_id, patient_id, &body)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?
        .ok_or_else(|| PdtApiError::NotFound("Plan de traitement introuvable".to_string()))?;

    // Update individual lignes if provided
    if let Some(ref ligne_updates) = body.lignes {
        for update in ligne_updates {
            update_ligne(&mut tx, pdt.id, update)
                .await
                .map_err(|e| PdtApiError::Database(e.to_string()))?;
        }
        // Recalculate prix_total after ligne updates
        recalculate_prix_total(&mut tx, pdt.id)
            .await
            .map_err(|e| PdtApiError::Database(e.to_string()))?;
    }

    let lignes = get_lignes_for_pdt(&mut tx, pdt.id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    // Re-fetch PDT to get updated prix_total
    let pdt = get_pdt_by_id(&mut tx, patient_id, pdt.id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?
        .ok_or_else(|| PdtApiError::NotFound("Plan de traitement introuvable".to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(PdtWithLines { pdt, lignes })))
}

// ─── POST /patients/:id/pdts/:pdt_id/pdf ──────────────────────────────────────

/// Generate a PDF stub — stores a placeholder URL and returns it.
pub async fn generate_pdf_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, pdt_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, PdtApiError> {
    if !auth_user.can_write_clinical() {
        return Err(PdtApiError::Forbidden);
    }

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    let stub_url = format!(
        "/api/v1/patients/{patient_id}/pdts/{pdt_id}/pdf/download"
    );

    let pdt = set_pdf_url(&mut tx, pdt_id, patient_id, &stub_url)
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?
        .ok_or_else(|| PdtApiError::NotFound("Plan de traitement introuvable".to_string()))?;

    tx.commit()
        .await
        .map_err(|e| PdtApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(
        serde_json::json!({ "pdf_url": pdt.pdf_url }),
    )))
}

// ─── Validation Helpers ───────────────────────────────────────────────────────

const VALID_FORMULES: &[&str] = &["lab", "compromis_1", "compromis_2", "budget"];
const VALID_STATUTS: &[&str] = &["brouillon", "presente", "accepte", "en_cours", "termine"];
const VALID_LIGNE_STATUTS: &[&str] = &["a_faire", "en_cours", "fait"];

fn validate_create_input(body: &CreatePdt) -> Result<(), PdtApiError> {
    if !VALID_FORMULES.contains(&body.formule.as_str()) {
        return Err(PdtApiError::Validation(format!(
            "Formule invalide: '{}'. Valeurs acceptées: lab, compromis_1, compromis_2, budget",
            body.formule
        )));
    }

    for l in &body.lignes {
        if l.prix_praticien < 0.0 {
            return Err(PdtApiError::Validation(
                "prix_praticien ne peut pas être négatif".to_string(),
            ));
        }
        if let Some(ref s) = l.statut {
            if !VALID_LIGNE_STATUTS.contains(&s.as_str()) {
                return Err(PdtApiError::Validation(format!(
                    "Statut de ligne invalide: '{s}'"
                )));
            }
        }
        if let Some(u) = l.urgence {
            if !(1..=5).contains(&u) {
                return Err(PdtApiError::Validation(
                    "urgence doit être entre 1 et 5".to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn validate_update_input(body: &UpdatePdt) -> Result<(), PdtApiError> {
    if let Some(ref s) = body.statut {
        if !VALID_STATUTS.contains(&s.as_str()) {
            return Err(PdtApiError::Validation(format!(
                "Statut invalide: '{s}'. Valeurs acceptées: brouillon, presente, accepte, en_cours, termine"
            )));
        }
    }

    if let Some(ref lignes) = body.lignes {
        for l in lignes {
            if let Some(ref s) = l.statut {
                if !VALID_LIGNE_STATUTS.contains(&s.as_str()) {
                    return Err(PdtApiError::Validation(format!(
                        "Statut de ligne invalide: '{s}'"
                    )));
                }
            }
            if let Some(u) = l.urgence {
                if !(1..=5).contains(&u) {
                    return Err(PdtApiError::Validation(
                        "urgence doit être entre 1 et 5".to_string(),
                    ));
                }
            }
        }
    }

    Ok(())
}

/// Calculate prix_total as SUM of all ligne prix_praticien.
fn calculate_prix_total(lignes: &[super::types::LignePdtInput]) -> BigDecimal {
    lignes
        .iter()
        .map(|l| {
            BigDecimal::from_str(&l.prix_praticien.to_string())
                .unwrap_or_else(|_| BigDecimal::from(0))
        })
        .fold(BigDecimal::from(0), |acc, x| acc + x)
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PdtApiError {
    NotFound(String),
    Forbidden,
    Conflict(String),
    Validation(String),
    Database(String),
}

impl IntoResponse for PdtApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PdtApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            PdtApiError::Forbidden => (StatusCode::FORBIDDEN, "Action non autorisée".to_string()),
            PdtApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            PdtApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            PdtApiError::Database(e) => {
                tracing::error!("Database error in pdt handler: {e}");
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
