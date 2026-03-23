use uuid::Uuid;

use super::types::{CreatePatient, Patient, UpdatePatient};

// ─── Patient Queries ─────────────────────────────────────────────────────────

/// Insert a new patient record within a tenant-scoped transaction.
pub async fn insert_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
    input: &CreatePatient,
) -> Result<Patient, sqlx::Error> {
    let row = sqlx::query_as!(
        Patient,
        r#"
        INSERT INTO patient (
            cabinet_id, nom, nom_naissance, prenom, sexe, date_naissance,
            num_ss, email, telephone, adresse, profession, couverture,
            mutuelle_nom, mutuelle_tableau_garantie,
            contact_urgence_nom, contact_urgence_tel, contact_urgence_lien,
            representant_legal_nom, representant_legal_tel,
            medecin_traitant_nom, medecin_traitant_tel
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12,
            $13, $14,
            $15, $16, $17,
            $18, $19,
            $20, $21
        )
        RETURNING
            id, cabinet_id, nom, nom_naissance, prenom, sexe, date_naissance,
            num_ss, email, telephone, adresse, profession, couverture,
            mutuelle_nom, mutuelle_tableau_garantie,
            contact_urgence_nom, contact_urgence_tel, contact_urgence_lien,
            representant_legal_nom, representant_legal_tel,
            medecin_traitant_nom, medecin_traitant_tel,
            created_at, updated_at, deleted_at
        "#,
        cabinet_id,
        input.nom,
        input.nom_naissance,
        input.prenom,
        input.sexe.as_str(),
        input.date_naissance,
        input.num_ss,
        input.email,
        input.telephone,
        input.adresse,
        input.profession,
        input.couverture.as_str(),
        input.mutuelle_nom,
        input.mutuelle_tableau_garantie,
        input.contact_urgence_nom,
        input.contact_urgence_tel,
        input.contact_urgence_lien,
        input.representant_legal_nom,
        input.representant_legal_tel,
        input.medecin_traitant_nom,
        input.medecin_traitant_tel,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row)
}

/// Fetch a single patient by ID (excludes soft-deleted).
pub async fn get_patient_by_id(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    id: Uuid,
) -> Result<Option<Patient>, sqlx::Error> {
    let row = sqlx::query_as!(
        Patient,
        r#"
        SELECT
            id, cabinet_id, nom, nom_naissance, prenom, sexe, date_naissance,
            num_ss, email, telephone, adresse, profession, couverture,
            mutuelle_nom, mutuelle_tableau_garantie,
            contact_urgence_nom, contact_urgence_tel, contact_urgence_lien,
            representant_legal_nom, representant_legal_tel,
            medecin_traitant_nom, medecin_traitant_tel,
            created_at, updated_at, deleted_at
        FROM patient
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id,
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

/// List patients with optional full-text search and pagination.
///
/// Returns the page of results and the total count of matching rows.
pub async fn list_patients(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    search: Option<&str>,
    page: i64,
    limit: i64,
) -> Result<(Vec<Patient>, i64), sqlx::Error> {
    let offset = (page - 1).max(0) * limit;
    let search_pattern = search.map(|s| format!("%{}%", s.to_lowercase()));

    let rows = sqlx::query_as!(
        Patient,
        r#"
        SELECT
            id, cabinet_id, nom, nom_naissance, prenom, sexe, date_naissance,
            num_ss, email, telephone, adresse, profession, couverture,
            mutuelle_nom, mutuelle_tableau_garantie,
            contact_urgence_nom, contact_urgence_tel, contact_urgence_lien,
            representant_legal_nom, representant_legal_tel,
            medecin_traitant_nom, medecin_traitant_tel,
            created_at, updated_at, deleted_at
        FROM patient
        WHERE deleted_at IS NULL
          AND (
            $1::TEXT IS NULL
            OR lower(nom) LIKE $1
            OR lower(prenom) LIKE $1
            OR lower(COALESCE(nom_naissance, '')) LIKE $1
            OR lower(COALESCE(num_ss, '')) LIKE $1
          )
        ORDER BY nom ASC, prenom ASC
        LIMIT $2 OFFSET $3
        "#,
        search_pattern,
        limit,
        offset,
    )
    .fetch_all(&mut **tx)
    .await?;

    let total: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM patient
        WHERE deleted_at IS NULL
          AND (
            $1::TEXT IS NULL
            OR lower(nom) LIKE $1
            OR lower(prenom) LIKE $1
            OR lower(COALESCE(nom_naissance, '')) LIKE $1
            OR lower(COALESCE(num_ss, '')) LIKE $1
          )
        "#,
        search_pattern,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok((rows, total))
}

/// Apply a partial update to a patient record.
///
/// Only non-None fields in `input` are updated; others remain unchanged.
pub async fn update_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    id: Uuid,
    input: &UpdatePatient,
) -> Result<Option<Patient>, sqlx::Error> {
    let row = sqlx::query_as!(
        Patient,
        r#"
        UPDATE patient SET
            nom                      = COALESCE($2, nom),
            nom_naissance            = COALESCE($3, nom_naissance),
            prenom                   = COALESCE($4, prenom),
            sexe                     = COALESCE($5, sexe),
            date_naissance           = COALESCE($6, date_naissance),
            num_ss                   = COALESCE($7, num_ss),
            email                    = COALESCE($8, email),
            telephone                = COALESCE($9, telephone),
            adresse                  = COALESCE($10, adresse),
            profession               = COALESCE($11, profession),
            couverture               = COALESCE($12, couverture),
            mutuelle_nom             = COALESCE($13, mutuelle_nom),
            mutuelle_tableau_garantie = COALESCE($14, mutuelle_tableau_garantie),
            contact_urgence_nom      = COALESCE($15, contact_urgence_nom),
            contact_urgence_tel      = COALESCE($16, contact_urgence_tel),
            contact_urgence_lien     = COALESCE($17, contact_urgence_lien),
            representant_legal_nom   = COALESCE($18, representant_legal_nom),
            representant_legal_tel   = COALESCE($19, representant_legal_tel),
            medecin_traitant_nom     = COALESCE($20, medecin_traitant_nom),
            medecin_traitant_tel     = COALESCE($21, medecin_traitant_tel)
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING
            id, cabinet_id, nom, nom_naissance, prenom, sexe, date_naissance,
            num_ss, email, telephone, adresse, profession, couverture,
            mutuelle_nom, mutuelle_tableau_garantie,
            contact_urgence_nom, contact_urgence_tel, contact_urgence_lien,
            representant_legal_nom, representant_legal_tel,
            medecin_traitant_nom, medecin_traitant_tel,
            created_at, updated_at, deleted_at
        "#,
        id,
        input.nom,
        input.nom_naissance,
        input.prenom,
        input.sexe.as_ref().map(|s| s.as_str()),
        input.date_naissance,
        input.num_ss,
        input.email,
        input.telephone,
        input.adresse,
        input.profession,
        input.couverture.as_ref().map(|c| c.as_str()),
        input.mutuelle_nom,
        input.mutuelle_tableau_garantie,
        input.contact_urgence_nom,
        input.contact_urgence_tel,
        input.contact_urgence_lien,
        input.representant_legal_nom,
        input.representant_legal_tel,
        input.medecin_traitant_nom,
        input.medecin_traitant_tel,
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

/// Soft-delete a patient by setting `deleted_at`.
///
/// Returns `true` if a row was found and marked deleted, `false` otherwise.
pub async fn soft_delete_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE patient
        SET deleted_at = now()
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id,
    )
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected() > 0)
}
