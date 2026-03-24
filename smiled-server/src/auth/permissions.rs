use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::AuthError;
use crate::auth::middleware::AuthUser;
use crate::state::AppState;

/// Check whether a user has a given permission.
///
/// Resolution order:
/// 1. If `user_permission_override` has an explicit grant/deny, use that.
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

/// Result of a permission check.
#[derive(Debug)]
pub enum PermissionDenied {
    /// User does not have the required permission.
    Forbidden,
    /// An internal error occurred while checking the permission.
    Internal(String),
}

/// Helper to enforce a specific permission on the current user.
///
/// Uses the moka cache on `AppState` to avoid repeated DB queries.
pub struct RequirePermission(pub &'static str);

impl RequirePermission {
    /// Check the permission. Returns `Ok(())` if allowed, `Err(PermissionDenied)` otherwise.
    pub async fn check(
        &self,
        state: &AppState,
        user: &AuthUser,
    ) -> Result<(), PermissionDenied> {
        let cache_key = format!("{}:{}", user.user_id, self.0);
        let pool = state.pool.clone();
        let user_id = user.user_id;
        let role = user.role.clone();
        let permission_code = self.0;

        let allowed = state
            .permission_cache
            .try_get_with(cache_key, async move {
                check_permission(&pool, user_id, &role, permission_code)
                    .await
                    .map_err(|e| e.to_string())
            })
            .await
            .map_err(|e| PermissionDenied::Internal(format!("Permission check failed: {e}")))?;

        if !allowed {
            return Err(PermissionDenied::Forbidden);
        }

        Ok(())
    }
}
