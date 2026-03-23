use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// ─── Diagnostic ───────────────────────────────────────────────────────────────

/// Full diagnostic row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Diagnostic {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub praticien_id: Uuid,
    pub schema_version: i32,
    pub date: NaiveDate,
    pub created_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// A clinical finding attached to a diagnostic.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Finding {
    pub id: Uuid,
    pub diagnostic_id: Uuid,
    #[serde(rename = "type")]
    pub type_: String,
    pub dent_fdi: Option<i16>,
    pub details: Option<Value>,
    pub severite: i16,
    pub source: String,
}

/// Diagnostic with all its findings, returned from GET endpoints.
#[derive(Debug, Serialize)]
pub struct DiagnosticWithFindings {
    #[serde(flatten)]
    pub diagnostic: Diagnostic,
    pub findings: Vec<Finding>,
}

// ─── Input Types ─────────────────────────────────────────────────────────────

/// A single finding to attach to a new diagnostic.
#[derive(Debug, Deserialize)]
pub struct FindingInput {
    #[serde(rename = "type")]
    pub type_: String,
    pub dent_fdi: Option<i16>,
    pub details: Option<Value>,
    pub severite: i16,
    pub source: String,
}

/// Input for creating a diagnostic with optional findings.
#[derive(Debug, Deserialize)]
pub struct CreateDiagnostic {
    pub schema_version: i32,
    pub date: Option<NaiveDate>,
    pub findings: Vec<FindingInput>,
}
