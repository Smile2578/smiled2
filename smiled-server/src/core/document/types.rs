use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// ─── Document types ──────────────────────────────────────────────────────────

/// Valid document type values.
pub const VALID_DOCUMENT_TYPES: &[&str] = &[
    "radio_retro",
    "panoramique",
    "cbct",
    "photo",
    "consentement",
    "devis",
    "ordonnance",
    "compte_rendu",
    "autre",
];

/// Full document row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub r#type: String,
    pub url_storage: String,
    pub filename: Option<String>,
    pub mime_type: Option<String>,
    pub uploaded_by: Uuid,
    pub uploaded_at: Option<DateTime<Utc>>,
}

/// Document with optional tooth links (from LEFT JOIN document_dent).
#[derive(Debug, Serialize)]
pub struct DocumentWithLinks {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub r#type: String,
    pub url_storage: String,
    pub filename: Option<String>,
    pub mime_type: Option<String>,
    pub uploaded_by: Uuid,
    pub uploaded_at: Option<DateTime<Utc>>,
    /// FDI tooth numbers linked to this document.
    pub dents_fdi: Vec<i16>,
}

/// Input for inserting a new document record.
pub struct CreateDocument {
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub document_type: String,
    pub url_storage: String,
    pub filename: Option<String>,
    pub mime_type: Option<String>,
    pub uploaded_by: Uuid,
}

/// Input from multipart form upload.
pub struct UploadInput {
    pub document_type: String,
    pub filename: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

