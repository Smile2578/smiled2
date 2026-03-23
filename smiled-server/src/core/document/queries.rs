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

/// List all documents for a patient with their linked FDI tooth numbers.
pub async fn list_documents_for_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<DocumentWithLinks>, sqlx::Error> {
    // Fetch documents
    let docs = sqlx::query_as!(
        Document,
        r#"
        SELECT id, patient_id, cabinet_id, type, url_storage,
               filename, mime_type, uploaded_by, uploaded_at
        FROM document
        WHERE patient_id = $1
        ORDER BY uploaded_at DESC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await?;

    // For each document, fetch its dent links
    let mut result = Vec::with_capacity(docs.len());

    for doc in docs {
        let dents = sqlx::query_scalar!(
            "SELECT dent_fdi FROM document_dent WHERE document_id = $1 ORDER BY dent_fdi",
            doc.id,
        )
        .fetch_all(&mut **tx)
        .await?;

        result.push(DocumentWithLinks {
            id: doc.id,
            patient_id: doc.patient_id,
            cabinet_id: doc.cabinet_id,
            r#type: doc.r#type,
            url_storage: doc.url_storage,
            filename: doc.filename,
            mime_type: doc.mime_type,
            uploaded_by: doc.uploaded_by,
            uploaded_at: doc.uploaded_at,
            dents_fdi: dents,
        });
    }

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
