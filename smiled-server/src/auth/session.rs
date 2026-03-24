use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::auth::AuthError;

/// Validated session data extracted from Better Auth's session store.
#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: Uuid,
    pub cabinet_id: Uuid,
    pub role: String,
    pub email: String,
}

/// Validate a Better Auth session token by querying the `auth.session` table.
///
/// Resolution:
/// 1. Look up `auth.session` WHERE `token = $1` AND `"expiresAt" > NOW()`
/// 2. JOIN `auth."user"` to get the auth user
/// 3. JOIN `public.utilisateur` via `auth_user_id` to get cabinet_id and role
///
/// Uses `sqlx::query()` (runtime) because `auth.*` tables are created by
/// Better Auth at runtime and don't exist at compile time.
pub async fn validate_session(
    pool: &PgPool,
    session_token: &str,
) -> Result<SessionUser, AuthError> {
    let row = sqlx::query(
        r#"
        SELECT
            u.id        AS user_id,
            u.cabinet_id,
            u.role,
            u.email
        FROM auth.session s
        JOIN auth."user" au ON au.id = s."userId"
        JOIN public.utilisateur u ON u.auth_user_id = au.id
        WHERE s.token = $1
          AND s."expiresAt" > NOW()
          AND u.actif = true
        "#,
    )
    .bind(session_token)
    .fetch_optional(pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?
    .ok_or(AuthError::InvalidSession)?;

    let user_id: Uuid = row
        .try_get("user_id")
        .map_err(|e| AuthError::Database(e.to_string()))?;
    let cabinet_id: Uuid = row
        .try_get("cabinet_id")
        .map_err(|e| AuthError::Database(e.to_string()))?;
    let role: String = row
        .try_get("role")
        .map_err(|e| AuthError::Database(e.to_string()))?;
    let email: String = row
        .try_get("email")
        .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(SessionUser {
        user_id,
        cabinet_id,
        role,
        email,
    })
}
