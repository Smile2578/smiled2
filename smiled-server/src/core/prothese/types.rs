use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─── Prothese Amovible ─────────────────────────────────────────────────────

/// Full prothese_amovible row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProtheseAmovible {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    #[serde(rename = "type")]
    pub type_: String,
    pub arcade: String,
    pub kennedy_class: Option<String>,
    pub kennedy_modifications: Option<i16>,
    pub dents_remplacees: Option<Vec<i16>>,
    pub crochets_sur: Option<Vec<i16>>,
    pub materiau_base_id: Option<Uuid>,
    pub date_pose: Option<NaiveDate>,
    pub etat: Option<String>,
    pub attachements: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Input for creating a prothese amovible.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProtheseAmovible {
    #[serde(rename = "type")]
    #[validate(length(min = 1, max = 100, message = "Le type est requis"))]
    pub type_: String,
    #[validate(length(min = 1, max = 50, message = "L'arcade est requise"))]
    pub arcade: String,
    pub kennedy_class: Option<String>,
    pub kennedy_modifications: Option<i16>,
    pub dents_remplacees: Option<Vec<i16>>,
    pub crochets_sur: Option<Vec<i16>>,
    pub materiau_base_id: Option<Uuid>,
    pub date_pose: Option<NaiveDate>,
    pub etat: Option<String>,
    pub attachements: Option<Vec<String>>,
}

/// Input for updating a prothese amovible.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProtheseAmovible {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub arcade: Option<String>,
    pub kennedy_class: Option<String>,
    pub kennedy_modifications: Option<i16>,
    pub dents_remplacees: Option<Vec<i16>>,
    pub crochets_sur: Option<Vec<i16>>,
    pub materiau_base_id: Option<Uuid>,
    pub date_pose: Option<NaiveDate>,
    pub etat: Option<String>,
    pub attachements: Option<Vec<String>>,
}

/// Valid arcade values.
pub const VALID_ARCADES: &[&str] = &["maxillaire", "mandibulaire"];
