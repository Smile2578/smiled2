use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Plan de Traitement ───────────────────────────────────────────────────────

/// Full plan_traitement row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Pdt {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub praticien_id: Uuid,
    pub diagnostic_id: Uuid,
    pub formule: String,
    pub statut: String,
    pub prix_total: Option<BigDecimal>,
    pub reste_a_charge_estime: Option<BigDecimal>,
    pub pdf_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// A single ligne_pdt row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LignePdt {
    pub id: Uuid,
    pub pdt_id: Uuid,
    pub acte_id: Option<Uuid>,
    pub dent_fdi: Option<i16>,
    pub faces: Option<Vec<String>>,
    pub materiau_id: Option<Uuid>,
    pub teinte_id: Option<Uuid>,
    pub urgence: Option<i16>,
    pub prix_praticien: BigDecimal,
    pub base_secu: Option<BigDecimal>,
    pub panier_rac: Option<String>,
    pub statut: String,
    pub ordre: i32,
}

/// PDT with all its lines — main response type.
#[derive(Debug, Serialize)]
pub struct PdtWithLines {
    #[serde(flatten)]
    pub pdt: Pdt,
    pub lignes: Vec<LignePdt>,
}

// ─── Input Types ─────────────────────────────────────────────────────────────

/// Input for a single ligne within a new PDT.
#[derive(Debug, Deserialize)]
pub struct LignePdtInput {
    pub acte_id: Option<Uuid>,
    pub dent_fdi: Option<i16>,
    pub faces: Option<Vec<String>>,
    pub materiau_id: Option<Uuid>,
    pub teinte_id: Option<Uuid>,
    pub urgence: Option<i16>,
    pub prix_praticien: f64,
    pub base_secu: Option<f64>,
    pub panier_rac: Option<String>,
    pub statut: Option<String>,
    pub ordre: i32,
}

/// Input for creating a new PDT with lines.
#[derive(Debug, Deserialize)]
pub struct CreatePdt {
    pub diagnostic_id: Uuid,
    pub formule: String,
    pub reste_a_charge_estime: Option<f64>,
    pub lignes: Vec<LignePdtInput>,
}

/// Input for updating an existing PDT (status change, lines update).
#[derive(Debug, Deserialize)]
pub struct UpdatePdt {
    pub statut: Option<String>,
    pub reste_a_charge_estime: Option<f64>,
    pub lignes: Option<Vec<UpdateLignePdt>>,
}

/// Line update within a PDT update — targets a specific ligne by ID.
#[derive(Debug, Deserialize)]
pub struct UpdateLignePdt {
    pub id: Uuid,
    pub statut: Option<String>,
    pub prix_praticien: Option<f64>,
    pub urgence: Option<i16>,
}
