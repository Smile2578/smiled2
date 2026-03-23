use uuid::Uuid;

use super::types::{Questionnaire, QuestionnaireInput};

// ─── Questionnaire Queries ────────────────────────────────────────────────────

/// Retrieve the latest questionnaire for a patient.
pub async fn get_questionnaire(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Option<Questionnaire>, sqlx::Error> {
    let row = sqlx::query_as!(
        Questionnaire,
        r#"
        SELECT
            id, patient_id, cabinet_id, version,
            date_signature, signe_par, nom_signataire, prochaine_maj,
            avk, aod_molecule, antiagregants, hemostase,
            endocardite, immunodepression, protheses_articulaires,
            bisphosphonates, antiresorptifs, radiotherapie,
            troubles, medicaments, allergies,
            tabac, alcool, drogues,
            grossesse_mois, allaitement, activite_physique,
            bruxisme, sahos, rgo, tca,
            dernier_rdv_date, brossage_quotidien,
            auxiliaires, historique_connu, apprehension,
            notice_information_date,
            created_at, updated_at
        FROM questionnaire_medical
        WHERE patient_id = $1
        ORDER BY version DESC
        LIMIT 1
        "#,
        patient_id,
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

/// Upsert a questionnaire for a patient.
///
/// If no questionnaire exists, inserts version 1.
/// Otherwise, inserts a new version (max + 1) to preserve history.
pub async fn upsert_questionnaire(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    cabinet_id: Uuid,
    input: &QuestionnaireInput,
) -> Result<Questionnaire, sqlx::Error> {
    // Atomic version increment: subquery computes next version within the INSERT
    // statement itself, eliminating the TOCTOU race of SELECT MAX + INSERT
    let row = sqlx::query_as!(
        Questionnaire,
        r#"
        INSERT INTO questionnaire_medical (
            patient_id, cabinet_id, version,
            date_signature, signe_par, nom_signataire, prochaine_maj,
            avk, aod_molecule, antiagregants, hemostase,
            endocardite, immunodepression, protheses_articulaires,
            bisphosphonates, antiresorptifs, radiotherapie,
            troubles, medicaments, allergies,
            tabac, alcool, drogues,
            grossesse_mois, allaitement, activite_physique,
            bruxisme, sahos, rgo, tca,
            dernier_rdv_date, brossage_quotidien,
            auxiliaires, historique_connu, apprehension,
            notice_information_date
        )
        VALUES (
            $1, $2,
            COALESCE((SELECT MAX(version) FROM questionnaire_medical WHERE patient_id = $1), 0) + 1,
            $3, $4, $5, $6,
            $7, $8, $9, $10,
            $11, $12, $13,
            $14, $15, $16,
            $17, $18, $19,
            $20, $21, $22,
            $23, $24, $25,
            $26, $27, $28, $29,
            $30, $31,
            $32, $33, $34,
            $35
        )
        RETURNING
            id, patient_id, cabinet_id, version,
            date_signature, signe_par, nom_signataire, prochaine_maj,
            avk, aod_molecule, antiagregants, hemostase,
            endocardite, immunodepression, protheses_articulaires,
            bisphosphonates, antiresorptifs, radiotherapie,
            troubles, medicaments, allergies,
            tabac, alcool, drogues,
            grossesse_mois, allaitement, activite_physique,
            bruxisme, sahos, rgo, tca,
            dernier_rdv_date, brossage_quotidien,
            auxiliaires, historique_connu, apprehension,
            notice_information_date,
            created_at, updated_at
        "#,
        patient_id,
        cabinet_id,
        input.date_signature,
        input.signe_par,
        input.nom_signataire,
        input.prochaine_maj,
        input.avk,
        input.aod_molecule,
        input.antiagregants,
        input.hemostase,
        input.endocardite,
        input.immunodepression,
        input.protheses_articulaires,
        input.bisphosphonates,
        input.antiresorptifs,
        input.radiotherapie,
        input.troubles,
        input.medicaments.as_deref(),
        input.allergies,
        input.tabac,
        input.alcool,
        input.drogues,
        input.grossesse_mois,
        input.allaitement,
        input.activite_physique,
        input.bruxisme,
        input.sahos,
        input.rgo,
        input.tca,
        input.dernier_rdv_date,
        input.brossage_quotidien,
        input.auxiliaires.as_deref(),
        input.historique_connu.as_deref(),
        input.apprehension,
        input.notice_information_date,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row)
}
