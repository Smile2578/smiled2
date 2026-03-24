use uuid::Uuid;

use super::types::{CreateProtheseAmovible, ProtheseAmovible, UpdateProtheseAmovible};

// ─── Prothese Amovible Queries ──────────────────────────────────────────────

/// List all non-deleted protheses amovibles for a patient.
pub async fn list_protheses(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<ProtheseAmovible>, sqlx::Error> {
    sqlx::query_as!(
        ProtheseAmovible,
        r#"
        SELECT id, patient_id, cabinet_id, type AS "type_",
               arcade, kennedy_class, kennedy_modifications,
               dents_remplacees, crochets_sur, materiau_base_id,
               date_pose, etat, attachements,
               created_at, updated_at, deleted_at
        FROM prothese_amovible
        WHERE patient_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Insert a new prothese amovible.
pub async fn insert_prothese(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    cabinet_id: Uuid,
    input: &CreateProtheseAmovible,
) -> Result<ProtheseAmovible, sqlx::Error> {
    sqlx::query_as!(
        ProtheseAmovible,
        r#"
        INSERT INTO prothese_amovible (
            patient_id, cabinet_id, type, arcade,
            kennedy_class, kennedy_modifications,
            dents_remplacees, crochets_sur, materiau_base_id,
            date_pose, etat, attachements
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, patient_id, cabinet_id, type AS "type_",
                  arcade, kennedy_class, kennedy_modifications,
                  dents_remplacees, crochets_sur, materiau_base_id,
                  date_pose, etat, attachements,
                  created_at, updated_at, deleted_at
        "#,
        patient_id,
        cabinet_id,
        input.type_,
        input.arcade,
        input.kennedy_class,
        input.kennedy_modifications,
        input.dents_remplacees.as_deref(),
        input.crochets_sur.as_deref(),
        input.materiau_base_id,
        input.date_pose,
        input.etat,
        input.attachements.as_deref(),
    )
    .fetch_one(&mut **tx)
    .await
}

/// Update a prothese amovible. Only non-None fields are applied.
pub async fn update_prothese(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    pa_id: Uuid,
    input: &UpdateProtheseAmovible,
) -> Result<Option<ProtheseAmovible>, sqlx::Error> {
    sqlx::query_as!(
        ProtheseAmovible,
        r#"
        UPDATE prothese_amovible SET
            type                  = COALESCE($3, type),
            arcade                = COALESCE($4, arcade),
            kennedy_class         = COALESCE($5, kennedy_class),
            kennedy_modifications = COALESCE($6, kennedy_modifications),
            dents_remplacees      = COALESCE($7, dents_remplacees),
            crochets_sur          = COALESCE($8, crochets_sur),
            materiau_base_id      = COALESCE($9, materiau_base_id),
            date_pose             = COALESCE($10, date_pose),
            etat                  = COALESCE($11, etat),
            attachements          = COALESCE($12, attachements)
        WHERE id = $2 AND patient_id = $1 AND deleted_at IS NULL
        RETURNING id, patient_id, cabinet_id, type AS "type_",
                  arcade, kennedy_class, kennedy_modifications,
                  dents_remplacees, crochets_sur, materiau_base_id,
                  date_pose, etat, attachements,
                  created_at, updated_at, deleted_at
        "#,
        patient_id,
        pa_id,
        input.type_,
        input.arcade,
        input.kennedy_class,
        input.kennedy_modifications,
        input.dents_remplacees.as_deref(),
        input.crochets_sur.as_deref(),
        input.materiau_base_id,
        input.date_pose,
        input.etat,
        input.attachements.as_deref(),
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Soft-delete a prothese amovible.
pub async fn soft_delete_prothese(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    pa_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE prothese_amovible
        SET deleted_at = now()
        WHERE id = $2 AND patient_id = $1 AND deleted_at IS NULL
        "#,
        patient_id,
        pa_id,
    )
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() > 0)
}
