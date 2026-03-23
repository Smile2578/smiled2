use chrono::Local;
use uuid::Uuid;

use super::types::{CreateDiagnostic, Diagnostic, Finding, FindingInput};

// ─── Diagnostic Queries ───────────────────────────────────────────────────────

/// Insert a new diagnostic row.
pub async fn insert_diagnostic(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    cabinet_id: Uuid,
    praticien_id: Uuid,
    input: &CreateDiagnostic,
) -> Result<Diagnostic, sqlx::Error> {
    let date = input.date.unwrap_or_else(|| Local::now().date_naive());

    sqlx::query_as!(
        Diagnostic,
        r#"
        INSERT INTO diagnostic (patient_id, cabinet_id, praticien_id, schema_version, date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, patient_id, cabinet_id, praticien_id, schema_version, date,
                  created_at, deleted_at
        "#,
        patient_id,
        cabinet_id,
        praticien_id,
        input.schema_version,
        date,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert all findings for a diagnostic in bulk.
pub async fn insert_findings(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    diagnostic_id: Uuid,
    findings: &[FindingInput],
) -> Result<Vec<Finding>, sqlx::Error> {
    let mut result = Vec::with_capacity(findings.len());

    for f in findings {
        let row = sqlx::query_as!(
            Finding,
            r#"
            INSERT INTO finding (diagnostic_id, type, dent_fdi, details, severite, source)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, diagnostic_id, type AS "type_", dent_fdi, details, severite, source
            "#,
            diagnostic_id,
            f.type_,
            f.dent_fdi,
            f.details,
            f.severite,
            f.source,
        )
        .fetch_one(&mut **tx)
        .await?;

        result.push(row);
    }

    Ok(result)
}

/// List all (non-deleted) diagnostics for a patient.
pub async fn list_diagnostics_for_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<Diagnostic>, sqlx::Error> {
    sqlx::query_as!(
        Diagnostic,
        r#"
        SELECT id, patient_id, cabinet_id, praticien_id, schema_version, date,
               created_at, deleted_at
        FROM diagnostic
        WHERE patient_id = $1 AND deleted_at IS NULL
        ORDER BY date DESC, created_at DESC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Get a single diagnostic by ID (must belong to the given patient).
pub async fn get_diagnostic_by_id(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    diagnostic_id: Uuid,
) -> Result<Option<Diagnostic>, sqlx::Error> {
    sqlx::query_as!(
        Diagnostic,
        r#"
        SELECT id, patient_id, cabinet_id, praticien_id, schema_version, date,
               created_at, deleted_at
        FROM diagnostic
        WHERE id = $1 AND patient_id = $2 AND deleted_at IS NULL
        "#,
        diagnostic_id,
        patient_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Get all findings for a diagnostic.
pub async fn get_findings_for_diagnostic(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    diagnostic_id: Uuid,
) -> Result<Vec<Finding>, sqlx::Error> {
    sqlx::query_as!(
        Finding,
        r#"
        SELECT id, diagnostic_id, type AS "type_", dent_fdi, details, severite, source
        FROM finding
        WHERE diagnostic_id = $1
        ORDER BY severite DESC
        "#,
        diagnostic_id,
    )
    .fetch_all(&mut **tx)
    .await
}
