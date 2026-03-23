use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::{
    auth::middleware::AuthUser,
    core::patient::types::ApiResponse,
    state::AppState,
    tenant::middleware::begin_tenant_transaction,
};

use super::{
    queries::{
        get_patient_exists, insert_document, insert_document_dent, list_documents_for_patient,
    },
    storage::build_storage_key,
    types::{CreateDocument, UploadInput, VALID_DOCUMENT_TYPES},
};

// ─── POST /patients/:id/documents/upload ──────────────────────────────────────

/// Upload a document file for a patient.
///
/// Expects multipart/form-data with:
/// - `file`: binary file field
/// - `type`: document type string
pub async fn upload_document_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, DocumentApiError> {
    if !auth_user.can_write_clinical() {
        return Err(DocumentApiError::Forbidden);
    }

    let upload = parse_multipart(&mut multipart).await?;

    validate_document_type(&upload.document_type)?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    // Verify patient exists and belongs to this cabinet
    let patient_exists = get_patient_exists(&mut tx, patient_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    if !patient_exists {
        return Err(DocumentApiError::PatientNotFound);
    }

    // Build storage key and upload
    let file_uuid = Uuid::new_v4();
    let key = build_storage_key(auth_user.cabinet_id, patient_id, file_uuid, &upload.filename);

    let url_storage = if let Some(ref storage) = state.s3 {
        storage
            .upload(&key, upload.data, &upload.mime_type)
            .await
            .map_err(|e| DocumentApiError::Storage(e.to_string()))?;
        key.clone()
    } else {
        // S3 not configured — store key as placeholder (useful in tests)
        key.clone()
    };

    let doc = insert_document(
        &mut tx,
        CreateDocument {
            patient_id,
            cabinet_id: auth_user.cabinet_id,
            document_type: upload.document_type,
            url_storage,
            filename: Some(upload.filename),
            mime_type: Some(upload.mime_type),
            uploaded_by: auth_user.user_id,
        },
    )
    .await
    .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(doc))))
}

// ─── GET /patients/:id/documents ──────────────────────────────────────────────

/// List all documents for a patient, including their tooth links.
pub async fn list_documents_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(patient_id): Path<Uuid>,
) -> Result<impl IntoResponse, DocumentApiError> {
    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    let patient_exists = get_patient_exists(&mut tx, patient_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    if !patient_exists {
        return Err(DocumentApiError::PatientNotFound);
    }

    let documents = list_documents_for_patient(&mut tx, patient_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    Ok(Json(ApiResponse::success(documents)))
}

// ─── POST /patients/:id/documents/:doc_id/link-dent/:fdi ─────────────────────

/// Link an existing document to a tooth by FDI number.
pub async fn link_dent_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((patient_id, doc_id, fdi)): Path<(Uuid, Uuid, i16)>,
) -> Result<impl IntoResponse, DocumentApiError> {
    if !auth_user.can_write_clinical() {
        return Err(DocumentApiError::Forbidden);
    }

    validate_fdi(fdi)?;

    let mut tx = begin_tenant_transaction(&state.pool, auth_user.cabinet_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    let patient_exists = get_patient_exists(&mut tx, patient_id)
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    if !patient_exists {
        return Err(DocumentApiError::PatientNotFound);
    }

    insert_document_dent(&mut tx, doc_id, fdi)
        .await
        .map_err(|e| {
            // Unique constraint violation → already linked
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                DocumentApiError::Conflict("Cette dent est déjà liée à ce document".to_string())
            } else if e.to_string().contains("foreign key") || e.to_string().contains("violates") {
                DocumentApiError::DocumentNotFound
            } else {
                DocumentApiError::Database(e.to_string())
            }
        })?;

    tx.commit()
        .await
        .map_err(|e| DocumentApiError::Database(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            serde_json::json!({ "document_id": doc_id, "dent_fdi": fdi }),
        )),
    ))
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Parse the multipart form and extract the file and type fields.
async fn parse_multipart(multipart: &mut Multipart) -> Result<UploadInput, DocumentApiError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename = "document".to_string();
    let mut mime_type = "application/octet-stream".to_string();
    let mut document_type: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| DocumentApiError::Validation(format!("Multipart parse error: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                if let Some(fname) = field.file_name() {
                    filename = fname.to_string();
                }
                if let Some(ct) = field.content_type() {
                    mime_type = ct.to_string();
                }
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| DocumentApiError::Validation(format!("Failed to read file: {e}")))?;
                file_data = Some(bytes.to_vec());
            }
            "type" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| DocumentApiError::Validation(format!("Failed to read type field: {e}")))?;
                document_type = Some(value);
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    let data = file_data.ok_or_else(|| {
        DocumentApiError::Validation("Champ 'file' manquant dans le formulaire".to_string())
    })?;

    let document_type = document_type.ok_or_else(|| {
        DocumentApiError::Validation("Champ 'type' manquant dans le formulaire".to_string())
    })?;

    Ok(UploadInput {
        document_type,
        filename,
        mime_type,
        data,
    })
}

fn validate_document_type(doc_type: &str) -> Result<(), DocumentApiError> {
    if !VALID_DOCUMENT_TYPES.contains(&doc_type) {
        return Err(DocumentApiError::Validation(format!(
            "Type de document invalide: '{}'. Valeurs acceptées: {}",
            doc_type,
            VALID_DOCUMENT_TYPES.join(", ")
        )));
    }
    Ok(())
}

/// FDI tooth numbering: primary teeth 51–85, permanent 11–48.
fn validate_fdi(fdi: i16) -> Result<(), DocumentApiError> {
    let valid = (11..=18).contains(&fdi)
        || (21..=28).contains(&fdi)
        || (31..=38).contains(&fdi)
        || (41..=48).contains(&fdi)
        || (51..=55).contains(&fdi)
        || (61..=65).contains(&fdi)
        || (71..=75).contains(&fdi)
        || (81..=85).contains(&fdi);

    if !valid {
        return Err(DocumentApiError::Validation(format!(
            "Numéro FDI invalide: {fdi}. Doit être compris dans la numérotation FDI (11-18, 21-28, 31-38, 41-48, 51-85)"
        )));
    }
    Ok(())
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DocumentApiError {
    PatientNotFound,
    DocumentNotFound,
    Forbidden,
    Conflict(String),
    Validation(String),
    Storage(String),
    Database(String),
}

impl IntoResponse for DocumentApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DocumentApiError::PatientNotFound => {
                (StatusCode::NOT_FOUND, "Patient introuvable".to_string())
            }
            DocumentApiError::DocumentNotFound => {
                (StatusCode::NOT_FOUND, "Document introuvable".to_string())
            }
            DocumentApiError::Forbidden => {
                (StatusCode::FORBIDDEN, "Action non autorisée".to_string())
            }
            DocumentApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            DocumentApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            DocumentApiError::Storage(e) => {
                tracing::error!("Storage error in document handler: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de stockage".to_string())
            }
            DocumentApiError::Database(e) => {
                tracing::error!("Database error in document handler: {e}");
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
