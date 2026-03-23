use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─── Catégorie matériau ───────────────────────────────────────────────────────

/// Flat category row from categorie_materiau.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CategorieMatriau {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub parent_id: Option<Uuid>,
}

// ─── Matériau ─────────────────────────────────────────────────────────────────

/// Full matériau row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Materiau {
    pub id: Uuid,
    pub categorie_materiau_id: Uuid,
    pub code: String,
    pub libelle: String,
    pub marques_courantes: Option<Vec<String>>,
    pub resistance_flexion_mpa: Option<i16>,
    pub obsolete: Option<bool>,
    pub note_reglementaire: Option<String>,
    pub niveau: String,
    pub cabinet_id: Option<Uuid>,
    pub actif: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
    /// Denormalized category code for convenience.
    pub categorie_code: Option<String>,
    /// Denormalized category label for convenience.
    pub categorie_libelle: Option<String>,
}

/// Input for creating a cabinet-specific matériau.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateMateriau {
    pub categorie_materiau_id: Uuid,
    #[validate(length(min = 1, max = 50, message = "Le code est requis"))]
    pub code: String,
    #[validate(length(min = 1, max = 300, message = "Le libellé est requis"))]
    pub libelle: String,
    pub marques_courantes: Option<Vec<String>>,
    pub resistance_flexion_mpa: Option<i16>,
    pub note_reglementaire: Option<String>,
}

/// Input for partial update of a matériau (cabinet-level only).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMateriau {
    pub categorie_materiau_id: Option<Uuid>,
    #[validate(length(min = 1, max = 300))]
    pub libelle: Option<String>,
    pub marques_courantes: Option<Vec<String>>,
    pub resistance_flexion_mpa: Option<i16>,
    pub note_reglementaire: Option<String>,
    pub actif: Option<bool>,
    pub obsolete: Option<bool>,
}

/// Query parameters for listing matériaux.
#[derive(Debug, Deserialize)]
pub struct MateriauListParams {
    pub categorie_id: Option<Uuid>,
    pub search: Option<String>,
    pub actif_only: Option<bool>,
}
