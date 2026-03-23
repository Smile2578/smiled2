use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthError;

/// Check whether a user has a given permission.
///
/// Resolution order:
/// 1. If `user_permission_override` has an explicit grant/deny → use that.
/// 2. Otherwise check `role_permission` for the user's role.
pub async fn check_permission(
    pool: &PgPool,
    user_id: Uuid,
    role: &str,
    permission_code: &str,
) -> Result<bool, AuthError> {
    // 1. Check user-level override first
    let override_row = sqlx::query!(
        r#"
        SELECT upo.granted
        FROM user_permission_override upo
        JOIN permission p ON p.id = upo.permission_id
        WHERE upo.user_id = $1
          AND p.code = $2
        "#,
        user_id,
        permission_code,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    if let Some(row) = override_row {
        return Ok(row.granted);
    }

    // 2. Fall back to role-level grant
    let role_row = sqlx::query!(
        r#"
        SELECT rp.role
        FROM role_permission rp
        JOIN permission p ON p.id = rp.permission_id
        WHERE rp.role = $1
          AND p.code = $2
        "#,
        role,
        permission_code,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;

    Ok(role_row.is_some())
}
