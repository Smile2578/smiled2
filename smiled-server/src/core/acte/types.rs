use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─── Acte ─────────────────────────────────────────────────────────────────────

/// Full acte row including optional cabinet tarif override.
///
/// NUMERIC columns are mapped as f64 via FLOAT8 casts in SQL.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Acte {
    pub id: Uuid,
    pub categorie_id: Uuid,
    pub nomenclature: String,
    pub code_ccam: Option<String>,
    pub code_ngap: Option<String>,
    pub libelle: String,
    pub base_secu: Option<f64>,
    pub prix_defaut: f64,
    pub panier_rac: Option<String>,
    pub plafond_rac_0: Option<f64>,
    pub modificateur_jeune: Option<bool>,
    pub dent_requise: Option<bool>,
    pub faces_requises: Option<bool>,
    pub teinte_requise: Option<bool>,
    pub nb_seances_typique: Option<i16>,
    pub niveau: String,
    pub cabinet_id: Option<Uuid>,
    pub actif: Option<bool>,
    pub date_entree_vigueur: Option<NaiveDate>,
    pub date_fin_vigueur: Option<NaiveDate>,
    pub notes_convention: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    /// Cabinet-specific price override (from tarif_cabinet), if any.
    pub prix_cabinet: Option<f64>,
}

/// Input for creating a cabinet-specific acte.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateActe {
    pub categorie_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub nomenclature: String,
    pub code_ccam: Option<String>,
    pub code_ngap: Option<String>,
    #[validate(length(min = 1, max = 500, message = "Le libellé est requis"))]
    pub libelle: String,
    #[validate(range(min = 0.0, message = "Le montant doit être positif ou nul"))]
    pub base_secu: Option<f64>,
    #[validate(range(min = 0.0, message = "Le prix doit être positif ou nul"))]
    pub prix_defaut: f64,
    pub panier_rac: Option<String>,
    #[validate(range(min = 0.0, message = "Le plafond doit être positif ou nul"))]
    pub plafond_rac_0: Option<f64>,
    pub modificateur_jeune: Option<bool>,
    pub dent_requise: Option<bool>,
    pub faces_requises: Option<bool>,
    pub teinte_requise: Option<bool>,
    pub nb_seances_typique: Option<i16>,
    pub date_entree_vigueur: Option<NaiveDate>,
    pub date_fin_vigueur: Option<NaiveDate>,
    pub notes_convention: Option<String>,
}

/// Input for partial update of an acte (cabinet-level only).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateActe {
    pub categorie_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub libelle: Option<String>,
    pub base_secu: Option<f64>,
    pub prix_defaut: Option<f64>,
    pub panier_rac: Option<String>,
    pub plafond_rac_0: Option<f64>,
    pub modificateur_jeune: Option<bool>,
    pub dent_requise: Option<bool>,
    pub faces_requises: Option<bool>,
    pub teinte_requise: Option<bool>,
    pub nb_seances_typique: Option<i16>,
    pub date_entree_vigueur: Option<NaiveDate>,
    pub date_fin_vigueur: Option<NaiveDate>,
    pub notes_convention: Option<String>,
}

/// Input for overriding the tarif of any acte for a specific cabinet.
#[derive(Debug, Deserialize, Validate)]
pub struct TarifOverride {
    #[validate(range(min = 0.0, message = "Le prix doit être positif ou nul"))]
    pub prix: f64,
}

/// Query parameters for listing actes.
#[derive(Debug, Deserialize)]
pub struct ActeListParams {
    pub categorie_id: Option<Uuid>,
    pub nomenclature: Option<String>,
    pub search: Option<String>,
    pub actif_only: Option<bool>,
}
