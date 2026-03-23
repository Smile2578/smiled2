use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use super::types::{CreatePdt, LignePdt, LignePdtInput, Pdt, UpdateLignePdt, UpdatePdt};

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn f64_to_decimal(v: f64) -> BigDecimal {
    BigDecimal::from_str(&v.to_string()).unwrap_or_else(|_| BigDecimal::from(0))
}

fn opt_f64_to_decimal(v: Option<f64>) -> Option<BigDecimal> {
    v.and_then(|n| BigDecimal::from_str(&n.to_string()).ok())
}

// ─── PDT Queries ──────────────────────────────────────────────────────────────

/// Check if a formule already exists for the given diagnostic (one per formule rule).
pub async fn formule_exists_for_diagnostic(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    diagnostic_id: Uuid,
    formule: &str,
) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM plan_traitement
        WHERE diagnostic_id = $1 AND formule = $2 AND deleted_at IS NULL
        "#,
        diagnostic_id,
        formule,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(count > 0)
}

/// Check if a diagnostic exists and belongs to the given patient.
pub async fn diagnostic_belongs_to_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    diagnostic_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM diagnostic
        WHERE id = $1 AND patient_id = $2 AND deleted_at IS NULL
        "#,
        diagnostic_id,
        patient_id,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(count > 0)
}

/// Insert a new plan_traitement row.
pub async fn insert_pdt(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    cabinet_id: Uuid,
    praticien_id: Uuid,
    input: &CreatePdt,
    prix_total: BigDecimal,
) -> Result<Pdt, sqlx::Error> {
    let reste = opt_f64_to_decimal(input.reste_a_charge_estime);

    sqlx::query_as!(
        Pdt,
        r#"
        INSERT INTO plan_traitement (
            patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime
        )
        VALUES ($1, $2, $3, $4, $5, 'brouillon', $6, $7)
        RETURNING
            id, patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime, pdf_url,
            created_at, updated_at, deleted_at
        "#,
        patient_id,
        cabinet_id,
        praticien_id,
        input.diagnostic_id,
        input.formule,
        prix_total,
        reste,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert all lignes for a PDT.
pub async fn insert_lignes(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
    lignes: &[LignePdtInput],
) -> Result<Vec<LignePdt>, sqlx::Error> {
    let mut result = Vec::with_capacity(lignes.len());

    for l in lignes {
        let prix = f64_to_decimal(l.prix_praticien);
        let base = opt_f64_to_decimal(l.base_secu);
        let statut = l.statut.as_deref().unwrap_or("a_faire");

        let row = sqlx::query_as!(
            LignePdt,
            r#"
            INSERT INTO ligne_pdt (
                pdt_id, acte_id, dent_fdi, faces, materiau_id, teinte_id,
                urgence, prix_praticien, base_secu, panier_rac, statut, ordre
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id, pdt_id, acte_id, dent_fdi, faces, materiau_id, teinte_id,
                urgence, prix_praticien, base_secu, panier_rac, statut, ordre
            "#,
            pdt_id,
            l.acte_id,
            l.dent_fdi,
            l.faces.as_deref(),
            l.materiau_id,
            l.teinte_id,
            l.urgence,
            prix,
            base,
            l.panier_rac,
            statut,
            l.ordre,
        )
        .fetch_one(&mut **tx)
        .await?;

        result.push(row);
    }

    Ok(result)
}

/// List all non-deleted PDTs for a patient.
pub async fn list_pdts_for_patient(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<Pdt>, sqlx::Error> {
    sqlx::query_as!(
        Pdt,
        r#"
        SELECT
            id, patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime, pdf_url,
            created_at, updated_at, deleted_at
        FROM plan_traitement
        WHERE patient_id = $1 AND deleted_at IS NULL
        ORDER BY created_at DESC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Get a single PDT by ID (must belong to the given patient).
pub async fn get_pdt_by_id(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    pdt_id: Uuid,
) -> Result<Option<Pdt>, sqlx::Error> {
    sqlx::query_as!(
        Pdt,
        r#"
        SELECT
            id, patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime, pdf_url,
            created_at, updated_at, deleted_at
        FROM plan_traitement
        WHERE id = $1 AND patient_id = $2 AND deleted_at IS NULL
        "#,
        pdt_id,
        patient_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Get all lignes for a PDT, ordered by display order.
pub async fn get_lignes_for_pdt(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
) -> Result<Vec<LignePdt>, sqlx::Error> {
    sqlx::query_as!(
        LignePdt,
        r#"
        SELECT
            id, pdt_id, acte_id, dent_fdi, faces, materiau_id, teinte_id,
            urgence, prix_praticien, base_secu, panier_rac, statut, ordre
        FROM ligne_pdt
        WHERE pdt_id = $1
        ORDER BY ordre ASC
        "#,
        pdt_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Update PDT fields (statut, reste_a_charge_estime).
pub async fn update_pdt_fields(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
    patient_id: Uuid,
    input: &UpdatePdt,
) -> Result<Option<Pdt>, sqlx::Error> {
    let reste = opt_f64_to_decimal(input.reste_a_charge_estime);

    sqlx::query_as!(
        Pdt,
        r#"
        UPDATE plan_traitement SET
            statut                 = COALESCE($3, statut),
            reste_a_charge_estime  = COALESCE($4, reste_a_charge_estime),
            updated_at             = now()
        WHERE id = $1 AND patient_id = $2 AND deleted_at IS NULL
        RETURNING
            id, patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime, pdf_url,
            created_at, updated_at, deleted_at
        "#,
        pdt_id,
        patient_id,
        input.statut,
        reste,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Update individual ligne fields.
pub async fn update_ligne(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
    update: &UpdateLignePdt,
) -> Result<Option<LignePdt>, sqlx::Error> {
    let prix = opt_f64_to_decimal(update.prix_praticien);

    sqlx::query_as!(
        LignePdt,
        r#"
        UPDATE ligne_pdt SET
            statut         = COALESCE($3, statut),
            prix_praticien = COALESCE($4, prix_praticien),
            urgence        = COALESCE($5, urgence)
        WHERE id = $1 AND pdt_id = $2
        RETURNING
            id, pdt_id, acte_id, dent_fdi, faces, materiau_id, teinte_id,
            urgence, prix_praticien, base_secu, panier_rac, statut, ordre
        "#,
        update.id,
        pdt_id,
        update.statut,
        prix,
        update.urgence,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Recalculate and persist prix_total from the current lignes.
pub async fn recalculate_prix_total(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
) -> Result<Option<BigDecimal>, sqlx::Error> {
    let total = sqlx::query_scalar!(
        r#"
        UPDATE plan_traitement
        SET prix_total = (
            SELECT COALESCE(SUM(prix_praticien), 0) FROM ligne_pdt WHERE pdt_id = $1
        ),
        updated_at = now()
        WHERE id = $1
        RETURNING prix_total
        "#,
        pdt_id,
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(total.flatten())
}

/// Store the PDF stub URL on the PDT row.
pub async fn set_pdf_url(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    pdt_id: Uuid,
    patient_id: Uuid,
    pdf_url: &str,
) -> Result<Option<Pdt>, sqlx::Error> {
    sqlx::query_as!(
        Pdt,
        r#"
        UPDATE plan_traitement
        SET pdf_url = $3, updated_at = now()
        WHERE id = $1 AND patient_id = $2 AND deleted_at IS NULL
        RETURNING
            id, patient_id, cabinet_id, praticien_id, diagnostic_id,
            formule, statut, prix_total, reste_a_charge_estime, pdf_url,
            created_at, updated_at, deleted_at
        "#,
        pdt_id,
        patient_id,
        pdf_url,
    )
    .fetch_optional(&mut **tx)
    .await
}
