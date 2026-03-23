use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

// ─── Dental Schema ────────────────────────────────────────────────────────────

/// Represents a versioned dental schema row in the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DentalSchema {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub version: i32,
    pub dentition: String,
    pub created_by: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

// ─── Tooth ────────────────────────────────────────────────────────────────────

/// Full tooth row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Tooth {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub numero_fdi: i16,
    pub successeur_fdi: Option<i16>,
    pub statut: String,
    pub eruption: Option<String>,
    pub prothese_fixe: Option<Value>,
    pub endo: Option<Value>,
    pub racine: Option<Value>,
    pub implant: Option<Value>,
    pub ortho: Option<Value>,
    pub traumatisme: Option<Value>,
    pub usure: Option<Value>,
    pub esthetique: Option<Value>,
    pub impaction: Option<Value>,
    pub paro_mobilite: Option<i16>,
    pub paro_furcation: Option<String>,
    pub paro_recession_class: Option<String>,
    pub media_ids: Option<Vec<Uuid>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── Tooth Face ───────────────────────────────────────────────────────────────

/// A single face of a tooth (M, D, O, V, P_L, C).
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ToothFace {
    pub id: Uuid,
    pub dent_id: Uuid,
    pub face: String,
    pub etat: String,
    pub restauration: Option<Value>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── Paro Site ────────────────────────────────────────────────────────────────

/// A periodontal site for a tooth (MB, B, DB, ML, L, DL).
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ParoSite {
    pub id: Uuid,
    pub dent_id: Uuid,
    pub site: String,
    pub profondeur_poche: Option<i16>,
    pub recession: Option<i16>,
    pub bop: Option<bool>,
    pub suppuration: Option<bool>,
    pub plaque: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── Occlusion ───────────────────────────────────────────────────────────────

/// Occlusion data — DECIMAL columns use `BigDecimal` via the sqlx `bigdecimal` feature.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Occlusion {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub angle_molaire_g: Option<String>,
    pub angle_molaire_d: Option<String>,
    pub angle_canine_g: Option<String>,
    pub angle_canine_d: Option<String>,
    pub overjet_mm: Option<BigDecimal>,
    pub overbite_mm: Option<BigDecimal>,
    pub beance: Option<String>,
    pub articule_croise: Option<Vec<i16>>,
    pub guidage_g: Option<String>,
    pub guidage_d: Option<String>,
    pub courbe_spee: Option<String>,
    pub dvo_mm: Option<BigDecimal>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── ATM ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Atm {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub ouverture_max_mm: Option<i16>,
    pub propulsion_mm: Option<i16>,
    pub lateralite_g_mm: Option<i16>,
    pub lateralite_d_mm: Option<i16>,
    pub bruit_g: Option<String>,
    pub bruit_d: Option<String>,
    pub deviation: Option<String>,
    pub douleur_musculaire: Option<Value>,
    pub blocage: Option<String>,
    pub score_helkimo: Option<i16>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── Paro Global ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ParoGlobal {
    pub id: Uuid,
    pub schema_id: Uuid,
    pub staging: Option<String>,
    pub grading: Option<String>,
    pub indice_plaque_pct: Option<i16>,
    pub bop_global_pct: Option<i16>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ─── Historique / Timeline ────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EvenementDent {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub dent_fdi: i16,
    pub date: NaiveDate,
    #[serde(rename = "type")]
    pub type_: String,
    pub acte_id: Option<Uuid>,
    pub description: Option<String>,
    pub praticien_id: Uuid,
    pub media_ids: Option<Vec<Uuid>>,
    pub schema_version: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

// ─── Composite Response Types ─────────────────────────────────────────────────

/// Full tooth entry with its faces and paro sites.
#[derive(Debug, Serialize)]
pub struct ToothEntry {
    pub tooth: Tooth,
    pub faces: Vec<ToothFace>,
    pub paro_sites: Vec<ParoSite>,
}

/// Full schema response including all nested data.
#[derive(Debug, Serialize)]
pub struct FullSchemaResponse {
    pub schema: DentalSchema,
    pub dents: Vec<ToothEntry>,
    pub occlusion: Option<Occlusion>,
    pub atm: Option<Atm>,
    pub paro_global: Option<ParoGlobal>,
}

// ─── Input Types ─────────────────────────────────────────────────────────────

/// Input for creating a new dental schema version.
#[derive(Debug, Deserialize)]
pub struct CreateSchemaInput {
    #[serde(default = "default_dentition")]
    pub dentition: String,
}

fn default_dentition() -> String {
    "permanente".to_string()
}

/// Query params for GET /schema (optional version selection).
#[derive(Debug, Deserialize)]
pub struct SchemaVersionQuery {
    pub version: Option<i32>,
}

/// Input for updating a single tooth.
#[derive(Debug, Deserialize)]
pub struct UpdateToothInput {
    pub statut: Option<String>,
    pub eruption: Option<String>,
    pub prothese_fixe: Option<Value>,
    pub endo: Option<Value>,
    pub racine: Option<Value>,
    pub implant: Option<Value>,
    pub ortho: Option<Value>,
    pub traumatisme: Option<Value>,
    pub usure: Option<Value>,
    pub esthetique: Option<Value>,
    pub impaction: Option<Value>,
    pub paro_mobilite: Option<i16>,
    pub paro_furcation: Option<String>,
    pub paro_recession_class: Option<String>,
}

/// A single paro site update within a bulk paro update.
#[derive(Debug, Deserialize)]
pub struct ParoSiteUpdate {
    pub dent_fdi: i16,
    pub site: String,
    pub profondeur_poche: Option<i16>,
    pub recession: Option<i16>,
    pub bop: Option<bool>,
    pub suppuration: Option<bool>,
    pub plaque: Option<bool>,
}

/// Input for bulk paro site update.
#[derive(Debug, Deserialize)]
pub struct UpdateParoInput {
    pub sites: Vec<ParoSiteUpdate>,
}

/// Input for updating occlusion data.
#[derive(Debug, Deserialize)]
pub struct UpdateOcclusionInput {
    pub angle_molaire_g: Option<String>,
    pub angle_molaire_d: Option<String>,
    pub angle_canine_g: Option<String>,
    pub angle_canine_d: Option<String>,
    pub overjet_mm: Option<f64>,
    pub overbite_mm: Option<f64>,
    pub beance: Option<String>,
    pub articule_croise: Option<Vec<i16>>,
    pub guidage_g: Option<String>,
    pub guidage_d: Option<String>,
    pub courbe_spee: Option<String>,
    pub dvo_mm: Option<f64>,
}

/// Input for updating ATM data.
#[derive(Debug, Deserialize)]
pub struct UpdateAtmInput {
    pub ouverture_max_mm: Option<i16>,
    pub propulsion_mm: Option<i16>,
    pub lateralite_g_mm: Option<i16>,
    pub lateralite_d_mm: Option<i16>,
    pub bruit_g: Option<String>,
    pub bruit_d: Option<String>,
    pub deviation: Option<String>,
    pub douleur_musculaire: Option<Value>,
    pub blocage: Option<String>,
    pub score_helkimo: Option<i16>,
}

/// Input for updating global paro summary.
#[derive(Debug, Deserialize)]
pub struct UpdateParoGlobalInput {
    pub staging: Option<String>,
    pub grading: Option<String>,
    pub indice_plaque_pct: Option<i16>,
    pub bop_global_pct: Option<i16>,
}

// ─── FDI Constants ───────────────────────────────────────────────────────────

/// Permanent dentition FDI numbers (32 teeth).
pub const FDI_PERMANENT: &[i16] = &[
    11, 12, 13, 14, 15, 16, 17, 18, // upper right
    21, 22, 23, 24, 25, 26, 27, 28, // upper left
    31, 32, 33, 34, 35, 36, 37, 38, // lower left
    41, 42, 43, 44, 45, 46, 47, 48, // lower right
];

/// Deciduous dentition FDI numbers (20 teeth).
pub const FDI_DECIDUOUS: &[i16] = &[
    51, 52, 53, 54, 55, // upper right deciduous
    61, 62, 63, 64, 65, // upper left deciduous
    71, 72, 73, 74, 75, // lower left deciduous
    81, 82, 83, 84, 85, // lower right deciduous
];

/// All tooth faces.
pub const TOOTH_FACES: &[&str] = &["M", "D", "O", "V", "P_L", "C"];

/// All paro sites.
pub const PARO_SITES: &[&str] = &["MB", "B", "DB", "ML", "L", "DL"];

/// Return FDI numbers for the requested dentition type.
pub fn fdi_for_dentition(dentition: &str) -> Vec<i16> {
    match dentition {
        "permanente" => FDI_PERMANENT.to_vec(),
        "lacteale" => FDI_DECIDUOUS.to_vec(),
        "mixte" => {
            let mut all = FDI_PERMANENT.to_vec();
            all.extend_from_slice(FDI_DECIDUOUS);
            all
        }
        _ => FDI_PERMANENT.to_vec(),
    }
}
