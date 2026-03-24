use uuid::Uuid;

use super::types::{Cabinet, CabinetUser};

// ─── Cabinet Queries ────────────────────────────────────────────────────────

/// Get cabinet by ID within a tenant-scoped transaction.
pub async fn get_cabinet(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
) -> Result<Option<Cabinet>, sqlx::Error> {
    sqlx::query_as!(
        Cabinet,
        r#"
        SELECT id, nom, adresse, siret, finess, plan, created_at, updated_at
        FROM cabinet
        WHERE id = $1
        "#,
        cabinet_id,
    )
    .fetch_optional(&mut **tx)
    .await
}

/// Update cabinet fields. Only non-None fields are applied.
pub async fn update_cabinet(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
    nom: Option<&str>,
    adresse: Option<&str>,
    siret: Option<&str>,
    finess: Option<&str>,
) -> Result<Option<Cabinet>, sqlx::Error> {
    sqlx::query_as!(
        Cabinet,
        r#"
        UPDATE cabinet SET
            nom     = COALESCE($2, nom),
            adresse = COALESCE($3, adresse),
            siret   = COALESCE($4, siret),
            finess  = COALESCE($5, finess)
        WHERE id = $1
        RETURNING id, nom, adresse, siret, finess, plan, created_at, updated_at
        "#,
        cabinet_id,
        nom,
        adresse,
        siret,
        finess,
    )
    .fetch_optional(&mut **tx)
    .await
}

// ─── User Queries ───────────────────────────────────────────────────────────

/// List all users belonging to a cabinet.
pub async fn list_cabinet_users(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
) -> Result<Vec<CabinetUser>, sqlx::Error> {
    sqlx::query_as!(
        CabinetUser,
        r#"
        SELECT id, cabinet_id, role, nom, prenom, email, rpps, actif,
               created_at, updated_at
        FROM utilisateur
        WHERE cabinet_id = $1
        ORDER BY nom ASC, prenom ASC
        "#,
        cabinet_id,
    )
    .fetch_all(&mut **tx)
    .await
}

/// Insert a new user (invite) into the cabinet.
/// Password is set to a placeholder; the user will set it later.
pub async fn insert_cabinet_user(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
    nom: &str,
    prenom: &str,
    email: &str,
    role: &str,
) -> Result<CabinetUser, sqlx::Error> {
    sqlx::query_as!(
        CabinetUser,
        r#"
        INSERT INTO utilisateur (cabinet_id, nom, prenom, email, role, password_hash, actif)
        VALUES ($1, $2, $3, $4, $5, '__INVITE_PENDING__', true)
        RETURNING id, cabinet_id, role, nom, prenom, email, rpps, actif,
                  created_at, updated_at
        "#,
        cabinet_id,
        nom,
        prenom,
        email,
        role,
    )
    .fetch_one(&mut **tx)
    .await
}

/// Update a user's role and/or actif status.
pub async fn update_cabinet_user(
    tx: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    cabinet_id: Uuid,
    user_id: Uuid,
    role: Option<&str>,
    actif: Option<bool>,
) -> Result<Option<CabinetUser>, sqlx::Error> {
    sqlx::query_as!(
        CabinetUser,
        r#"
        UPDATE utilisateur SET
            role  = COALESCE($3, role),
            actif = COALESCE($4, actif)
        WHERE id = $2 AND cabinet_id = $1
        RETURNING id, cabinet_id, role, nom, prenom, email, rpps, actif,
                  created_at, updated_at
        "#,
        cabinet_id,
        user_id,
        role,
        actif,
    )
    .fetch_optional(&mut **tx)
    .await
}
