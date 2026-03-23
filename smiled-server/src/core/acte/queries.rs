use uuid::Uuid;

use super::types::{Acte, ActeListParams, CreateActe, UpdateActe};

// ─── Acte Queries ─────────────────────────────────────────────────────────────

/// List all actes visible for a cabinet:
/// - All système actes (shared)
/// - Cabinet-specific actes (WHERE cabinet_id = current)
/// - LEFT JOIN tarif_cabinet to surface per-cabinet price override
///
/// NUMERIC columns are cast to FLOAT8 (f64) — sqlx `bigdecimal` feature not needed.
pub async fn list_actes(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    params: &ActeListParams,
) -> Result<Vec<Acte>, sqlx::Error> {
    let actif_filter = params.actif_only.unwrap_or(true);

    sqlx::query_as!(
        Acte,
        r#"
        SELECT
            a.id,
            a.categorie_id,
            a.nomenclature,
            a.code_ccam,
            a.code_ngap,
            a.libelle,
            a.base_secu::FLOAT8       AS "base_secu?",
            a.prix_defaut::FLOAT8     AS "prix_defaut!",
            a.panier_rac,
            a.plafond_rac_0::FLOAT8   AS "plafond_rac_0?",
            a.modificateur_jeune,
            a.dent_requise,
            a.faces_requises,
            a.teinte_requise,
            a.nb_seances_typique,
            a.niveau,
            a.cabinet_id,
            a.actif,
            a.date_entree_vigueur,
            a.date_fin_vigueur,
            a.notes_convention,
            a.updated_at,
            tc.prix::FLOAT8           AS "prix_cabinet?"
        FROM acte a
        LEFT JOIN tarif_cabinet tc
            ON tc.acte_id = a.id AND tc.cabinet_id = $1
        WHERE
            (a.niveau = 'systeme' OR a.cabinet_id = $1)
            AND ($2::BOOLEAN = false OR a.actif = true)
            AND ($3::UUID IS NULL OR a.categorie_id = $3)
            AND ($4::TEXT IS NULL OR a.nomenclature = $4)
            AND ($5::TEXT IS NULL OR lower(a.libelle) LIKE '%' || lower($5) || '%')
        ORDER BY a.nomenclature ASC, a.libelle ASC
        "#,
        cabinet_id,
        actif_filter,
        params.categorie_id,
        params.nomenclature,
        params.search,
    )
    .fetch_all(pool)
    .await
}

/// Insert a new cabinet-level acte.
pub async fn insert_acte(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    input: &CreateActe,
) -> Result<Acte, sqlx::Error> {
    sqlx::query_as!(
        Acte,
        r#"
        INSERT INTO acte (
            categorie_id, nomenclature, code_ccam, code_ngap, libelle,
            base_secu, prix_defaut, panier_rac, plafond_rac_0,
            modificateur_jeune, dent_requise, faces_requises, teinte_requise,
            nb_seances_typique, niveau, cabinet_id,
            date_entree_vigueur, date_fin_vigueur, notes_convention
        )
        VALUES (
            $1, $2, $3, $4, $5,
            $6::FLOAT8, $7::FLOAT8, $8, $9::FLOAT8,
            $10, $11, $12, $13,
            $14, 'cabinet', $15,
            $16, $17, $18
        )
        RETURNING
            id, categorie_id, nomenclature, code_ccam, code_ngap, libelle,
            base_secu::FLOAT8     AS "base_secu?",
            prix_defaut::FLOAT8   AS "prix_defaut!",
            panier_rac,
            plafond_rac_0::FLOAT8 AS "plafond_rac_0?",
            modificateur_jeune, dent_requise, faces_requises, teinte_requise,
            nb_seances_typique, niveau, cabinet_id, actif,
            date_entree_vigueur, date_fin_vigueur, notes_convention, updated_at,
            NULL::FLOAT8 AS "prix_cabinet?"
        "#,
        input.categorie_id,
        input.nomenclature,
        input.code_ccam,
        input.code_ngap,
        input.libelle,
        input.base_secu,
        input.prix_defaut,
        input.panier_rac,
        input.plafond_rac_0,
        input.modificateur_jeune,
        input.dent_requise,
        input.faces_requises,
        input.teinte_requise,
        input.nb_seances_typique,
        cabinet_id,
        input.date_entree_vigueur,
        input.date_fin_vigueur,
        input.notes_convention,
    )
    .fetch_one(pool)
    .await
}

/// Update a cabinet-level acte (system actes cannot be modified).
pub async fn update_acte(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
    input: &UpdateActe,
) -> Result<Option<Acte>, sqlx::Error> {
    sqlx::query_as!(
        Acte,
        r#"
        UPDATE acte SET
            categorie_id        = COALESCE($3, categorie_id),
            libelle             = COALESCE($4, libelle),
            base_secu           = COALESCE($5::FLOAT8, base_secu),
            prix_defaut         = COALESCE($6::FLOAT8, prix_defaut),
            panier_rac          = COALESCE($7, panier_rac),
            plafond_rac_0       = COALESCE($8::FLOAT8, plafond_rac_0),
            modificateur_jeune  = COALESCE($9, modificateur_jeune),
            dent_requise        = COALESCE($10, dent_requise),
            faces_requises      = COALESCE($11, faces_requises),
            teinte_requise      = COALESCE($12, teinte_requise),
            nb_seances_typique  = COALESCE($13, nb_seances_typique),
            date_entree_vigueur = COALESCE($14, date_entree_vigueur),
            date_fin_vigueur    = COALESCE($15, date_fin_vigueur),
            notes_convention    = COALESCE($16, notes_convention)
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet'
        RETURNING
            id, categorie_id, nomenclature, code_ccam, code_ngap, libelle,
            base_secu::FLOAT8     AS "base_secu?",
            prix_defaut::FLOAT8   AS "prix_defaut!",
            panier_rac,
            plafond_rac_0::FLOAT8 AS "plafond_rac_0?",
            modificateur_jeune, dent_requise, faces_requises, teinte_requise,
            nb_seances_typique, niveau, cabinet_id, actif,
            date_entree_vigueur, date_fin_vigueur, notes_convention, updated_at,
            NULL::FLOAT8 AS "prix_cabinet?"
        "#,
        id,
        cabinet_id,
        input.categorie_id,
        input.libelle,
        input.base_secu,
        input.prix_defaut,
        input.panier_rac,
        input.plafond_rac_0,
        input.modificateur_jeune,
        input.dent_requise,
        input.faces_requises,
        input.teinte_requise,
        input.nb_seances_typique,
        input.date_entree_vigueur,
        input.date_fin_vigueur,
        input.notes_convention,
    )
    .fetch_optional(pool)
    .await
}

/// Toggle the `actif` flag on a cabinet-level acte.
pub async fn toggle_acte(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
) -> Result<Option<Acte>, sqlx::Error> {
    sqlx::query_as!(
        Acte,
        r#"
        UPDATE acte SET actif = NOT actif
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet'
        RETURNING
            id, categorie_id, nomenclature, code_ccam, code_ngap, libelle,
            base_secu::FLOAT8     AS "base_secu?",
            prix_defaut::FLOAT8   AS "prix_defaut!",
            panier_rac,
            plafond_rac_0::FLOAT8 AS "plafond_rac_0?",
            modificateur_jeune, dent_requise, faces_requises, teinte_requise,
            nb_seances_typique, niveau, cabinet_id, actif,
            date_entree_vigueur, date_fin_vigueur, notes_convention, updated_at,
            NULL::FLOAT8 AS "prix_cabinet?"
        "#,
        id,
        cabinet_id,
    )
    .fetch_optional(pool)
    .await
}

/// Upsert a cabinet tarif override for any acte (including système actes).
pub async fn upsert_tarif(
    pool: &sqlx::PgPool,
    acte_id: Uuid,
    cabinet_id: Uuid,
    prix: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO tarif_cabinet (cabinet_id, acte_id, prix, actif)
        VALUES ($1, $2, $3::FLOAT8, true)
        ON CONFLICT (cabinet_id, acte_id)
        DO UPDATE SET prix = EXCLUDED.prix, actif = true
        "#,
        cabinet_id,
        acte_id,
        prix,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Verify that an acte exists (either système or belonging to the cabinet).
pub async fn acte_exists(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) AS "count!"
        FROM acte
        WHERE id = $1 AND (niveau = 'systeme' OR cabinet_id = $2)
        "#,
        id,
        cabinet_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}
