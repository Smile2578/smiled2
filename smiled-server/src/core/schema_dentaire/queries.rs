use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

use super::types::{
    Atm, DentalSchema, EvenementDent, Occlusion, ParoGlobal, ParoSite, ParoSiteUpdate, Tooth,
    ToothFace, UpdateAtmInput, UpdateOcclusionInput, UpdateParoGlobalInput, UpdateToothInput,
    fdi_for_dentition, PARO_SITES, TOOTH_FACES,
};

/// Convert an optional f64 to an optional BigDecimal.
fn f64_to_decimal(v: Option<f64>) -> Option<BigDecimal> {
    v.and_then(|n| BigDecimal::from_str(&n.to_string()).ok())
}

// ─── Schema Creation ──────────────────────────────────────────────────────────

/// Insert a new schema_dentaire row using atomic version increment.
pub async fn insert_schema(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    cabinet_id: Uuid,
    created_by: Uuid,
    dentition: &str,
) -> Result<DentalSchema, sqlx::Error> {
    sqlx::query_as!(
        DentalSchema,
        r#"
        INSERT INTO schema_dentaire (patient_id, cabinet_id, version, dentition, created_by)
        VALUES (
            $1, $2,
            COALESCE((SELECT MAX(version) FROM schema_dentaire WHERE patient_id = $1), 0) + 1,
            $3, $4
        )
        RETURNING id, patient_id, cabinet_id, version, dentition, created_by, created_at
        "#,
        patient_id,
        cabinet_id,
        dentition,
        created_by,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert a single tooth row into a schema.
pub async fn insert_tooth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    numero_fdi: i16,
) -> Result<Tooth, sqlx::Error> {
    sqlx::query_as!(
        Tooth,
        r#"
        INSERT INTO dent (schema_id, numero_fdi)
        VALUES ($1, $2)
        RETURNING
            id, schema_id, numero_fdi, successeur_fdi, statut, eruption,
            prothese_fixe, endo, racine, implant, ortho, traumatisme,
            usure, esthetique, impaction,
            paro_mobilite, paro_furcation, paro_recession_class,
            media_ids, updated_at
        "#,
        schema_id,
        numero_fdi,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert all 6 faces for a tooth.
pub async fn insert_tooth_faces(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    tooth_id: Uuid,
) -> Result<Vec<ToothFace>, sqlx::Error> {
    // Build unnest arrays to insert all 6 faces in a single statement
    let tooth_ids: Vec<Uuid> = TOOTH_FACES.iter().map(|_| tooth_id).collect();
    let faces: Vec<&str> = TOOTH_FACES.to_vec();

    sqlx::query_as!(
        ToothFace,
        r#"
        INSERT INTO face_dent (dent_id, face)
        SELECT * FROM UNNEST($1::uuid[], $2::text[])
        RETURNING id, dent_id, face, etat, restauration, updated_at
        "#,
        &tooth_ids as &[Uuid],
        &faces as &[&str],
    )
    .fetch_all(&mut **tx)
    .await
}

/// Insert all 6 paro sites for a tooth.
pub async fn insert_paro_sites(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    tooth_id: Uuid,
) -> Result<Vec<ParoSite>, sqlx::Error> {
    let tooth_ids: Vec<Uuid> = PARO_SITES.iter().map(|_| tooth_id).collect();
    let sites: Vec<&str> = PARO_SITES.to_vec();

    sqlx::query_as!(
        ParoSite,
        r#"
        INSERT INTO paro_site (dent_id, site)
        SELECT * FROM UNNEST($1::uuid[], $2::text[])
        RETURNING id, dent_id, site, profondeur_poche, recession, bop, suppuration, plaque, updated_at
        "#,
        &tooth_ids as &[Uuid],
        &sites as &[&str],
    )
    .fetch_all(&mut **tx)
    .await
}

/// Insert an empty occlusion row for a schema.
pub async fn insert_occlusion(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Occlusion, sqlx::Error> {
    sqlx::query_as!(
        Occlusion,
        r#"
        INSERT INTO occlusion (schema_id)
        VALUES ($1)
        RETURNING
            id, schema_id,
            angle_molaire_g, angle_molaire_d, angle_canine_g, angle_canine_d,
            overjet_mm, overbite_mm,
            beance, articule_croise, guidage_g, guidage_d, courbe_spee,
            dvo_mm, updated_at
        "#,
        schema_id,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert an empty ATM row for a schema.
pub async fn insert_atm(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Atm, sqlx::Error> {
    sqlx::query_as!(
        Atm,
        r#"
        INSERT INTO atm (schema_id)
        VALUES ($1)
        RETURNING
            id, schema_id, ouverture_max_mm, propulsion_mm,
            lateralite_g_mm, lateralite_d_mm, bruit_g, bruit_d,
            deviation, douleur_musculaire, blocage, score_helkimo, updated_at
        "#,
        schema_id,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Insert an empty paro_global row for a schema.
pub async fn insert_paro_global(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<ParoGlobal, sqlx::Error> {
    sqlx::query_as!(
        ParoGlobal,
        r#"
        INSERT INTO paro_global (schema_id)
        VALUES ($1)
        RETURNING id, schema_id, staging, grading, indice_plaque_pct, bop_global_pct, updated_at
        "#,
        schema_id,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Initialize all teeth (with faces and paro sites) for a newly created schema.
///
/// Returns the list of teeth IDs inserted.
pub async fn init_schema_teeth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    dentition: &str,
) -> Result<Vec<Uuid>, sqlx::Error> {
    let fdi_numbers = fdi_for_dentition(dentition);
    let mut tooth_ids = Vec::with_capacity(fdi_numbers.len());

    for fdi in fdi_numbers {
        let tooth = insert_tooth(tx, schema_id, fdi).await?;
        let tooth_id = tooth.id;
        insert_tooth_faces(tx, tooth_id).await?;
        insert_paro_sites(tx, tooth_id).await?;
        tooth_ids.push(tooth_id);
    }

    Ok(tooth_ids)
}

// ─── Schema Queries ───────────────────────────────────────────────────────────

/// Fetch the latest (or specific version) schema for a patient.
pub async fn get_schema(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    version: Option<i32>,
) -> Result<Option<DentalSchema>, sqlx::Error> {
    sqlx::query_as!(
        DentalSchema,
        r#"
        SELECT id, patient_id, cabinet_id, version, dentition, created_by, created_at
        FROM schema_dentaire
        WHERE patient_id = $1
          AND ($2::INT IS NULL OR version = $2)
        ORDER BY version DESC
        LIMIT 1
        "#,
        patient_id,
        version,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// List all schema versions for a patient (id + version + created_at).
pub async fn list_schema_versions(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<Vec<DentalSchema>, sqlx::Error> {
    sqlx::query_as!(
        DentalSchema,
        r#"
        SELECT id, patient_id, cabinet_id, version, dentition, created_by, created_at
        FROM schema_dentaire
        WHERE patient_id = $1
        ORDER BY version ASC
        "#,
        patient_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Fetch all teeth for a schema.
pub async fn get_teeth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Vec<Tooth>, sqlx::Error> {
    sqlx::query_as!(
        Tooth,
        r#"
        SELECT
            id, schema_id, numero_fdi, successeur_fdi, statut, eruption,
            prothese_fixe, endo, racine, implant, ortho, traumatisme,
            usure, esthetique, impaction,
            paro_mobilite, paro_furcation, paro_recession_class,
            media_ids, updated_at
        FROM dent
        WHERE schema_id = $1
        ORDER BY numero_fdi ASC
        "#,
        schema_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Fetch all faces for a list of tooth IDs.
pub async fn get_faces_for_teeth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    tooth_ids: &[Uuid],
) -> Result<Vec<ToothFace>, sqlx::Error> {
    sqlx::query_as!(
        ToothFace,
        r#"
        SELECT id, dent_id, face, etat, restauration, updated_at
        FROM face_dent
        WHERE dent_id = ANY($1)
        ORDER BY dent_id, face
        "#,
        tooth_ids,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Fetch all paro sites for a list of tooth IDs.
pub async fn get_paro_sites_for_teeth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    tooth_ids: &[Uuid],
) -> Result<Vec<ParoSite>, sqlx::Error> {
    sqlx::query_as!(
        ParoSite,
        r#"
        SELECT id, dent_id, site, profondeur_poche, recession, bop, suppuration, plaque, updated_at
        FROM paro_site
        WHERE dent_id = ANY($1)
        ORDER BY dent_id, site
        "#,
        tooth_ids,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Fetch a single tooth by schema_id + FDI number.
pub async fn get_tooth_by_fdi(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    fdi: i16,
) -> Result<Option<Tooth>, sqlx::Error> {
    sqlx::query_as!(
        Tooth,
        r#"
        SELECT
            id, schema_id, numero_fdi, successeur_fdi, statut, eruption,
            prothese_fixe, endo, racine, implant, ortho, traumatisme,
            usure, esthetique, impaction,
            paro_mobilite, paro_furcation, paro_recession_class,
            media_ids, updated_at
        FROM dent
        WHERE schema_id = $1 AND numero_fdi = $2
        "#,
        schema_id,
        fdi,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── Tooth Update ─────────────────────────────────────────────────────────────

/// Partially update a tooth's clinical properties.
pub async fn update_tooth(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    tooth_id: Uuid,
    input: &UpdateToothInput,
) -> Result<Option<Tooth>, sqlx::Error> {
    sqlx::query_as!(
        Tooth,
        r#"
        UPDATE dent SET
            statut               = COALESCE($2, statut),
            eruption             = COALESCE($3, eruption),
            prothese_fixe        = COALESCE($4, prothese_fixe),
            endo                 = COALESCE($5, endo),
            racine               = COALESCE($6, racine),
            implant              = COALESCE($7, implant),
            ortho                = COALESCE($8, ortho),
            traumatisme          = COALESCE($9, traumatisme),
            usure                = COALESCE($10, usure),
            esthetique           = COALESCE($11, esthetique),
            impaction            = COALESCE($12, impaction),
            paro_mobilite        = COALESCE($13, paro_mobilite),
            paro_furcation       = COALESCE($14, paro_furcation),
            paro_recession_class = COALESCE($15, paro_recession_class)
        WHERE id = $1
        RETURNING
            id, schema_id, numero_fdi, successeur_fdi, statut, eruption,
            prothese_fixe, endo, racine, implant, ortho, traumatisme,
            usure, esthetique, impaction,
            paro_mobilite, paro_furcation, paro_recession_class,
            media_ids, updated_at
        "#,
        tooth_id,
        input.statut,
        input.eruption,
        input.prothese_fixe,
        input.endo,
        input.racine,
        input.implant,
        input.ortho,
        input.traumatisme,
        input.usure,
        input.esthetique,
        input.impaction,
        input.paro_mobilite,
        input.paro_furcation,
        input.paro_recession_class,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── Paro Bulk Update ─────────────────────────────────────────────────────────

/// Bulk upsert paro site measurements. Resolves tooth IDs from FDI + schema_id.
pub async fn bulk_update_paro(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    updates: &[ParoSiteUpdate],
) -> Result<Vec<ParoSite>, sqlx::Error> {
    let mut updated = Vec::with_capacity(updates.len());

    for update in updates {
        let site = sqlx::query_as!(
            ParoSite,
            r#"
            UPDATE paro_site ps SET
                profondeur_poche = COALESCE($4, ps.profondeur_poche),
                recession        = COALESCE($5, ps.recession),
                bop              = COALESCE($6, ps.bop),
                suppuration      = COALESCE($7, ps.suppuration),
                plaque           = COALESCE($8, ps.plaque)
            FROM dent d
            WHERE ps.dent_id = d.id
              AND d.schema_id = $1
              AND d.numero_fdi = $2
              AND ps.site = $3
            RETURNING ps.id, ps.dent_id, ps.site, ps.profondeur_poche,
                      ps.recession, ps.bop, ps.suppuration, ps.plaque, ps.updated_at
            "#,
            schema_id,
            update.dent_fdi,
            update.site,
            update.profondeur_poche,
            update.recession,
            update.bop,
            update.suppuration,
            update.plaque,
        )
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(s) = site {
            updated.push(s);
        }
    }

    Ok(updated)
}

/// Fetch all paro sites for the current schema (via tooth join).
pub async fn get_all_paro_for_schema(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Vec<ParoSite>, sqlx::Error> {
    sqlx::query_as!(
        ParoSite,
        r#"
        SELECT ps.id, ps.dent_id, ps.site, ps.profondeur_poche,
               ps.recession, ps.bop, ps.suppuration, ps.plaque, ps.updated_at
        FROM paro_site ps
        JOIN dent d ON ps.dent_id = d.id
        WHERE d.schema_id = $1
        ORDER BY d.numero_fdi, ps.site
        "#,
        schema_id,
    )
    .fetch_all(&mut **tx)
    .await
}

// ─── Occlusion Queries ────────────────────────────────────────────────────────

/// Fetch occlusion for a schema.
pub async fn get_occlusion(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Option<Occlusion>, sqlx::Error> {
    sqlx::query_as!(
        Occlusion,
        r#"
        SELECT
            id, schema_id,
            angle_molaire_g, angle_molaire_d, angle_canine_g, angle_canine_d,
            overjet_mm, overbite_mm,
            beance, articule_croise, guidage_g, guidage_d, courbe_spee,
            dvo_mm, updated_at
        FROM occlusion
        WHERE schema_id = $1
        "#,
        schema_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Update occlusion for a schema.
pub async fn update_occlusion(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    input: &UpdateOcclusionInput,
) -> Result<Option<Occlusion>, sqlx::Error> {
    let overjet = f64_to_decimal(input.overjet_mm);
    let overbite = f64_to_decimal(input.overbite_mm);
    let dvo = f64_to_decimal(input.dvo_mm);

    sqlx::query_as!(
        Occlusion,
        r#"
        UPDATE occlusion SET
            angle_molaire_g = COALESCE($2, angle_molaire_g),
            angle_molaire_d = COALESCE($3, angle_molaire_d),
            angle_canine_g  = COALESCE($4, angle_canine_g),
            angle_canine_d  = COALESCE($5, angle_canine_d),
            overjet_mm      = COALESCE($6, overjet_mm),
            overbite_mm     = COALESCE($7, overbite_mm),
            beance          = COALESCE($8, beance),
            articule_croise = COALESCE($9, articule_croise),
            guidage_g       = COALESCE($10, guidage_g),
            guidage_d       = COALESCE($11, guidage_d),
            courbe_spee     = COALESCE($12, courbe_spee),
            dvo_mm          = COALESCE($13, dvo_mm)
        WHERE schema_id = $1
        RETURNING
            id, schema_id,
            angle_molaire_g, angle_molaire_d, angle_canine_g, angle_canine_d,
            overjet_mm, overbite_mm,
            beance, articule_croise, guidage_g, guidage_d, courbe_spee,
            dvo_mm, updated_at
        "#,
        schema_id,
        input.angle_molaire_g,
        input.angle_molaire_d,
        input.angle_canine_g,
        input.angle_canine_d,
        overjet,
        overbite,
        input.beance,
        input.articule_croise.as_deref(),
        input.guidage_g,
        input.guidage_d,
        input.courbe_spee,
        dvo,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── ATM Queries ──────────────────────────────────────────────────────────────

/// Fetch ATM for a schema.
pub async fn get_atm(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Option<Atm>, sqlx::Error> {
    sqlx::query_as!(
        Atm,
        r#"
        SELECT
            id, schema_id, ouverture_max_mm, propulsion_mm,
            lateralite_g_mm, lateralite_d_mm, bruit_g, bruit_d,
            deviation, douleur_musculaire, blocage, score_helkimo, updated_at
        FROM atm
        WHERE schema_id = $1
        "#,
        schema_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Update ATM data for a schema.
pub async fn update_atm(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    input: &UpdateAtmInput,
) -> Result<Option<Atm>, sqlx::Error> {
    sqlx::query_as!(
        Atm,
        r#"
        UPDATE atm SET
            ouverture_max_mm   = COALESCE($2, ouverture_max_mm),
            propulsion_mm      = COALESCE($3, propulsion_mm),
            lateralite_g_mm    = COALESCE($4, lateralite_g_mm),
            lateralite_d_mm    = COALESCE($5, lateralite_d_mm),
            bruit_g            = COALESCE($6, bruit_g),
            bruit_d            = COALESCE($7, bruit_d),
            deviation          = COALESCE($8, deviation),
            douleur_musculaire = COALESCE($9, douleur_musculaire),
            blocage            = COALESCE($10, blocage),
            score_helkimo      = COALESCE($11, score_helkimo)
        WHERE schema_id = $1
        RETURNING
            id, schema_id, ouverture_max_mm, propulsion_mm,
            lateralite_g_mm, lateralite_d_mm, bruit_g, bruit_d,
            deviation, douleur_musculaire, blocage, score_helkimo, updated_at
        "#,
        schema_id,
        input.ouverture_max_mm,
        input.propulsion_mm,
        input.lateralite_g_mm,
        input.lateralite_d_mm,
        input.bruit_g,
        input.bruit_d,
        input.deviation,
        input.douleur_musculaire,
        input.blocage,
        input.score_helkimo,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── Paro Global Queries ──────────────────────────────────────────────────────

/// Fetch paro_global for a schema.
pub async fn get_paro_global(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
) -> Result<Option<ParoGlobal>, sqlx::Error> {
    sqlx::query_as!(
        ParoGlobal,
        r#"
        SELECT id, schema_id, staging, grading, indice_plaque_pct, bop_global_pct, updated_at
        FROM paro_global
        WHERE schema_id = $1
        "#,
        schema_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Update paro_global for a schema.
pub async fn update_paro_global(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    schema_id: Uuid,
    input: &UpdateParoGlobalInput,
) -> Result<Option<ParoGlobal>, sqlx::Error> {
    sqlx::query_as!(
        ParoGlobal,
        r#"
        UPDATE paro_global SET
            staging           = COALESCE($2, staging),
            grading           = COALESCE($3, grading),
            indice_plaque_pct = COALESCE($4, indice_plaque_pct),
            bop_global_pct    = COALESCE($5, bop_global_pct)
        WHERE schema_id = $1
        RETURNING id, schema_id, staging, grading, indice_plaque_pct, bop_global_pct, updated_at
        "#,
        schema_id,
        input.staging,
        input.grading,
        input.indice_plaque_pct,
        input.bop_global_pct,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── Timeline Query ───────────────────────────────────────────────────────────

/// Fetch the clinical event timeline for a specific tooth (patient + FDI).
pub async fn get_tooth_timeline(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
    dent_fdi: i16,
) -> Result<Vec<EvenementDent>, sqlx::Error> {
    sqlx::query_as!(
        EvenementDent,
        r#"
        SELECT
            id, patient_id, cabinet_id, dent_fdi, date,
            type as "type_",
            acte_id, description, praticien_id, media_ids, schema_version, created_at
        FROM evenement_dent
        WHERE patient_id = $1 AND dent_fdi = $2
        ORDER BY date DESC, created_at DESC
        "#,
        patient_id,
        dent_fdi,
    )
    .fetch_all(&mut **tx)
    .await
}

// ─── Patient Existence Check ──────────────────────────────────────────────────

/// Check if a patient exists and belongs to the current cabinet (via RLS).
pub async fn patient_exists(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    patient_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM patient WHERE id = $1 AND deleted_at IS NULL",
        patient_id,
    )
    .fetch_one(&mut **tx)
    .await?
    .unwrap_or(0);

    Ok(count > 0)
}
