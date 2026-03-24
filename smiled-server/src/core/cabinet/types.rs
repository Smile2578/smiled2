use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ─── Cabinet ────────────────────────────────────────────────────────────────

/// Full cabinet row as returned from the database.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Cabinet {
    pub id: Uuid,
    pub nom: String,
    pub adresse: Option<String>,
    pub siret: Option<String>,
    pub finess: Option<String>,
    pub plan: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Input for updating cabinet info.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCabinet {
    #[validate(length(min = 1, max = 300, message = "Le nom est requis"))]
    pub nom: Option<String>,
    pub adresse: Option<String>,
    pub siret: Option<String>,
    pub finess: Option<String>,
}

// ─── Utilisateur (cabinet user) ─────────────────────────────────────────────

/// User row as returned from cabinet user listings.
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CabinetUser {
    pub id: Uuid,
    pub cabinet_id: Uuid,
    pub role: String,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub rpps: Option<String>,
    pub actif: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Input for inviting a new user to the cabinet.
#[derive(Debug, Deserialize, Validate)]
pub struct InviteUser {
    #[validate(length(min = 1, max = 200, message = "Le nom est requis"))]
    pub nom: String,
    #[validate(length(min = 1, max = 200, message = "Le prénom est requis"))]
    pub prenom: String,
    #[validate(email(message = "Email invalide"))]
    pub email: String,
    #[validate(length(min = 1, max = 50, message = "Le rôle est requis"))]
    pub role: String,
}

/// Input for updating a user's role and/or actif status.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    pub role: Option<String>,
    pub actif: Option<bool>,
}

/// Valid roles for utilisateur.
pub const VALID_ROLES: &[&str] = &[
    "titulaire",
    "associe",
    "collaborateur",
    "remplacant",
    "specialiste_odf",
    "specialiste_co",
    "specialiste_mbd",
    "assistant",
    "assistant_formation",
    "aspbd",
    "secretaire",
    "comptable",
    "admin",
    "prothesiste",
];
