use uuid::Uuid;

use super::types::{CategorieMatriau, CreateMateriau, Materiau, MateriauListParams, UpdateMateriau};

// ─── Matériau Queries ─────────────────────────────────────────────────────────

/// List all matériaux visible for a cabinet with category info.
pub async fn list_materiaux(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    params: &MateriauListParams,
) -> Result<Vec<Materiau>, sqlx::Error> {
    let actif_filter = params.actif_only.unwrap_or(true);

    sqlx::query_as!(
        Materiau,
        r#"
        SELECT
            m.id,
            m.categorie_materiau_id,
            m.code,
            m.libelle,
            m.marques_courantes,
            m.resistance_flexion_mpa,
            m.obsolete,
            m.note_reglementaire,
            m.niveau,
            m.cabinet_id,
            m.actif,
            m.updated_at,
            cm.code AS categorie_code,
            cm.libelle AS categorie_libelle
        FROM materiau m
        JOIN categorie_materiau cm ON cm.id = m.categorie_materiau_id
        WHERE
            (m.niveau = 'systeme' OR m.cabinet_id = $1)
            AND ($2::BOOLEAN = false OR m.actif = true)
            AND ($3::UUID IS NULL OR m.categorie_materiau_id = $3)
            AND ($4::TEXT IS NULL OR lower(m.libelle) LIKE '%' || lower($4) || '%')
        ORDER BY cm.libelle ASC, m.libelle ASC
        "#,
        cabinet_id,
        actif_filter,
        params.categorie_id,
        params.search,
    )
    .fetch_all(pool)
    .await
}

/// List all matériau categories (flat list, hierarchy via parent_id).
pub async fn list_categories(pool: &sqlx::PgPool) -> Result<Vec<CategorieMatriau>, sqlx::Error> {
    sqlx::query_as!(
        CategorieMatriau,
        r#"
        SELECT id, code, libelle, parent_id
        FROM categorie_materiau
        ORDER BY libelle ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

/// Insert a new cabinet-level matériau.
pub async fn insert_materiau(
    pool: &sqlx::PgPool,
    cabinet_id: Uuid,
    input: &CreateMateriau,
) -> Result<Materiau, sqlx::Error> {
    sqlx::query_as!(
        Materiau,
        r#"
        INSERT INTO materiau (
            categorie_materiau_id, code, libelle,
            marques_courantes, resistance_flexion_mpa,
            note_reglementaire, niveau, cabinet_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, 'cabinet', $7)
        RETURNING
            id, categorie_materiau_id, code, libelle,
            marques_courantes, resistance_flexion_mpa, obsolete,
            note_reglementaire, niveau, cabinet_id, actif, updated_at,
            NULL::TEXT AS categorie_code,
            NULL::TEXT AS categorie_libelle
        "#,
        input.categorie_materiau_id,
        input.code,
        input.libelle,
        input.marques_courantes.as_deref(),
        input.resistance_flexion_mpa,
        input.note_reglementaire,
        cabinet_id,
    )
    .fetch_one(pool)
    .await
}

/// Update a cabinet-level matériau.
pub async fn update_materiau(
    pool: &sqlx::PgPool,
    id: Uuid,
    cabinet_id: Uuid,
    input: &UpdateMateriau,
) -> Result<Option<Materiau>, sqlx::Error> {
    sqlx::query_as!(
        Materiau,
        r#"
        UPDATE materiau SET
            categorie_materiau_id = COALESCE($3, categorie_materiau_id),
            libelle               = COALESCE($4, libelle),
            marques_courantes     = COALESCE($5, marques_courantes),
            resistance_flexion_mpa = COALESCE($6, resistance_flexion_mpa),
            note_reglementaire    = COALESCE($7, note_reglementaire),
            actif                 = COALESCE($8, actif),
            obsolete              = COALESCE($9, obsolete)
        WHERE id = $1 AND cabinet_id = $2 AND niveau = 'cabinet'
        RETURNING
            id, categorie_materiau_id, code, libelle,
            marques_courantes, resistance_flexion_mpa, obsolete,
            note_reglementaire, niveau, cabinet_id, actif, updated_at,
            NULL::TEXT AS categorie_code,
            NULL::TEXT AS categorie_libelle
        "#,
        id,
        cabinet_id,
        input.categorie_materiau_id,
        input.libelle,
        input.marques_courantes.as_deref(),
        input.resistance_flexion_mpa,
        input.note_reglementaire,
        input.actif,
        input.obsolete,
    )
    .fetch_optional(pool)
    .await
}
