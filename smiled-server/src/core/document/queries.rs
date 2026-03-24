use uuid::Uuid;

use super::types::{CreateDocument, Document, DocumentWithLinks};

// ─── Patient existence check ──────────────────────────────────────────────────

/// Return true if a non-deleted patient with this ID exists within the current RLS context.
pub async fn get_patient_exists(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM patient WHERE id = $1 AND deleted_at IS NULL",
        patient_id,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.unwrap_or(0) > 0)
}

// ─── Insert document ─────────────────────────────────────────────────────────

/// Insert a new document record and return it.
pub async fn insert_document(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    input: CreateDocument,
) -> Result<Document, sqlx::Error> {
    sqlx::query_as!(
        Document,
        r#"
        INSERT INTO document (
            patient_id, cabinet_id, type, url_storage,
            filename, mime_type, uploaded_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            id, patient_id, cabinet_id, type,
            url_storage, filename, mime_type,
            uploaded_by, uploaded_at
        "#,
        input.patient_id,
        input.cabinet_id,
        input.document_type,
        input.url_storage,
        input.filename,
        input.mime_type,
        input.uploaded_by,
    )
    .fetch_one(&mut **tx)
    .await
}

// ─── List documents for patient ──────────────────────────────────────────────

/// Intermediate row for the single-query documents + linked teeth fetch.
#[derive(sqlx::FromRow)]
struct DocumentWithLinkedTeethRow {
    id: Uuid,
    patient_id: Uuid,
    cabinet_id: Uuid,
    r#type: String,
    url_storage: String,
    filename: Option<String>,
    mime_type: Option<String>,
    uploaded_by: Uuid,
    uploaded_at: Option<chrono::DateTime<chrono::Utc>>,
    linked_teeth: Vec<i16>,
}

/// List all documents for a patient with their linked FDI tooth numbers.
/// Uses a single query with LEFT JOIN + array_agg to avoid N+1.
pub async fn list_documents_for_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<DocumentWithLinks>, sqlx::Error> {
    let rows = sqlx::query_as!(
        DocumentWithLinkedTeethRow,
        r#"
        SELECT
            d.id, d.patient_id, d.cabinet_id, d.type, d.url_storage,
            d.filename, d.mime_type, d.uploaded_by, d.uploaded_at,
            COALESCE(
                array_agg(dd.dent_fdi ORDER BY dd.dent_fdi) FILTER (WHERE dd.dent_fdi IS NOT NULL),
                '{}'
            ) as "linked_teeth!: Vec<i16>"
        FROM document d
        LEFT JOIN document_dent dd ON dd.document_id = d.id
        WHERE d.patient_id = $1
        GROUP BY d.id
        ORDER BY d.uploaded_at DESC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await?;

    let result = rows
        .into_iter()
        .map(|row| DocumentWithLinks {
            id: row.id,
            patient_id: row.patient_id,
            cabinet_id: row.cabinet_id,
            r#type: row.r#type,
            url_storage: row.url_storage,
            filename: row.filename,
            mime_type: row.mime_type,
            uploaded_by: row.uploaded_by,
            uploaded_at: row.uploaded_at,
            dents_fdi: row.linked_teeth,
        })
        .collect();

    Ok(result)
}

// ─── Link document to tooth ───────────────────────────────────────────────────

/// Insert a document_dent link record.
pub async fn insert_document_dent(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    document_id: Uuid,
    dent_fdi: i16,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO document_dent (document_id, dent_fdi)
        VALUES ($1, $2)
        ON CONFLICT (document_id, dent_fdi) DO NOTHING
        "#,
        document_id,
        dent_fdi,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
