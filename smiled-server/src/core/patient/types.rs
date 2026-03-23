use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use validator::Validate;

// ─── Patient ─────────────────────────────────────────────────────────────────

/// Full patient row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Patient {
    pub id: Uuid,
    pub cabinet_id: Uuid,
    pub nom: String,
    pub nom_naissance: Option<String>,
    pub prenom: String,
    pub sexe: String,
    pub date_naissance: NaiveDate,
    pub num_ss: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub profession: Option<String>,
    pub couverture: String,
    pub mutuelle_nom: Option<String>,
    pub mutuelle_tableau_garantie: Option<bool>,
    pub contact_urgence_nom: Option<String>,
    pub contact_urgence_tel: Option<String>,
    pub contact_urgence_lien: Option<String>,
    pub representant_legal_nom: Option<String>,
    pub representant_legal_tel: Option<String>,
    pub medecin_traitant_nom: Option<String>,
    pub medecin_traitant_tel: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Input for creating a new patient.
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePatient {
    #[validate(length(min = 1, max = 200, message = "Le nom est requis (1-200 caractères)"))]
    pub nom: String,
    pub nom_naissance: Option<String>,
    #[validate(length(min = 1, max = 200, message = "Le prénom est requis (1-200 caractères)"))]
    pub prenom: String,
    pub sexe: Sexe,
    pub date_naissance: NaiveDate,
    pub num_ss: Option<String>,
    #[validate(email(message = "Adresse email invalide"))]
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub profession: Option<String>,
    pub couverture: Couverture,
    pub mutuelle_nom: Option<String>,
    pub mutuelle_tableau_garantie: Option<bool>,
    pub contact_urgence_nom: Option<String>,
    pub contact_urgence_tel: Option<String>,
    pub contact_urgence_lien: Option<String>,
    pub representant_legal_nom: Option<String>,
    pub representant_legal_tel: Option<String>,
    pub medecin_traitant_nom: Option<String>,
    pub medecin_traitant_tel: Option<String>,
}

/// Input for partial update of an existing patient.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePatient {
    #[validate(length(min = 1, max = 200, message = "Le nom est requis (1-200 caractères)"))]
    pub nom: Option<String>,
    pub nom_naissance: Option<String>,
    #[validate(length(min = 1, max = 200, message = "Le prénom est requis (1-200 caractères)"))]
    pub prenom: Option<String>,
    pub sexe: Option<Sexe>,
    pub date_naissance: Option<NaiveDate>,
    pub num_ss: Option<String>,
    #[validate(email(message = "Adresse email invalide"))]
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub profession: Option<String>,
    pub couverture: Option<Couverture>,
    pub mutuelle_nom: Option<String>,
    pub mutuelle_tableau_garantie: Option<bool>,
    pub contact_urgence_nom: Option<String>,
    pub contact_urgence_tel: Option<String>,
    pub contact_urgence_lien: Option<String>,
    pub representant_legal_nom: Option<String>,
    pub representant_legal_tel: Option<String>,
    pub medecin_traitant_nom: Option<String>,
    pub medecin_traitant_tel: Option<String>,
}

/// Sex enum — validated from DB CHECK constraint values.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Sexe {
    #[serde(rename = "M")]
    Masculin,
    #[serde(rename = "F")]
    Feminin,
}

impl Sexe {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sexe::Masculin => "M",
            Sexe::Feminin => "F",
        }
    }
}

/// Insurance coverage type.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Couverture {
    #[serde(rename = "mutuelle")]
    Mutuelle,
    #[serde(rename = "cmu_c2s")]
    CmuC2s,
    #[serde(rename = "ame")]
    Ame,
    #[serde(rename = "aucune")]
    Aucune,
}

impl Couverture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Couverture::Mutuelle => "mutuelle",
            Couverture::CmuC2s => "cmu_c2s",
            Couverture::Ame => "ame",
            Couverture::Aucune => "aucune",
        }
    }
}

/// Query parameters for listing / searching patients.
#[derive(Debug, Deserialize)]
pub struct PatientListParams {
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

// ─── Questionnaire ────────────────────────────────────────────────────────────

/// Full questionnaire row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Questionnaire {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub cabinet_id: Uuid,
    pub version: i32,
    pub date_signature: Option<NaiveDate>,
    pub signe_par: Option<String>,
    pub nom_signataire: Option<String>,
    pub prochaine_maj: Option<NaiveDate>,
    pub avk: Option<Value>,
    pub aod_molecule: Option<String>,
    pub antiagregants: Option<Value>,
    pub hemostase: Option<Value>,
    pub endocardite: Option<Value>,
    pub immunodepression: Option<Value>,
    pub protheses_articulaires: Option<Value>,
    pub bisphosphonates: Option<Value>,
    pub antiresorptifs: Option<Value>,
    pub radiotherapie: Option<Value>,
    pub troubles: Option<Value>,
    pub medicaments: Option<Vec<String>>,
    pub allergies: Option<Value>,
    pub tabac: Option<String>,
    pub alcool: Option<String>,
    pub drogues: Option<Value>,
    pub grossesse_mois: Option<i32>,
    pub allaitement: Option<bool>,
    pub activite_physique: Option<String>,
    pub bruxisme: Option<String>,
    pub sahos: Option<Value>,
    pub rgo: Option<bool>,
    pub tca: Option<String>,
    pub dernier_rdv_date: Option<NaiveDate>,
    pub brossage_quotidien: Option<i16>,
    pub auxiliaires: Option<Vec<String>>,
    pub historique_connu: Option<Vec<String>>,
    pub apprehension: Option<String>,
    pub notice_information_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Input for creating or updating a medical questionnaire.
#[derive(Debug, Deserialize)]
pub struct QuestionnaireInput {
    pub date_signature: Option<NaiveDate>,
    pub signe_par: Option<String>,
    pub nom_signataire: Option<String>,
    pub prochaine_maj: Option<NaiveDate>,
    // Haemostasis / haemorrhage risk (P0)
    pub avk: Option<Value>,
    pub aod_molecule: Option<String>,
    pub antiagregants: Option<Value>,
    pub hemostase: Option<Value>,
    // Infectious risk (P0)
    pub endocardite: Option<Value>,
    pub immunodepression: Option<Value>,
    pub protheses_articulaires: Option<Value>,
    // Drug risk (P0)
    pub bisphosphonates: Option<Value>,
    pub antiresorptifs: Option<Value>,
    pub radiotherapie: Option<Value>,
    // General medical troubles
    pub troubles: Option<Value>,
    // Medications & allergies
    pub medicaments: Option<Vec<String>>,
    pub allergies: Option<Value>,
    // Lifestyle habits
    pub tabac: Option<String>,
    pub alcool: Option<String>,
    pub drogues: Option<Value>,
    pub grossesse_mois: Option<i32>,
    pub allaitement: Option<bool>,
    pub activite_physique: Option<String>,
    pub bruxisme: Option<String>,
    pub sahos: Option<Value>,
    pub rgo: Option<bool>,
    pub tca: Option<String>,
    // Dental history
    pub dernier_rdv_date: Option<NaiveDate>,
    pub brossage_quotidien: Option<i16>,
    pub auxiliaires: Option<Vec<String>>,
    pub historique_connu: Option<Vec<String>>,
    pub apprehension: Option<String>,
    // RGPD
    pub notice_information_date: Option<NaiveDate>,
}

// ─── API response wrappers ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: None,
        }
    }

    pub fn success_with_meta(data: T, meta: PaginationMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: Some(meta),
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            meta: None,
        }
    }
}
