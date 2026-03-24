use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::middleware::extract_session_token, state::AppState};

/// Tower middleware that logs mutating requests (POST/PUT/PATCH/DELETE) to the
/// `audit_log` table after the response has been produced.
///
/// The INSERT is fire-and-forget (`tokio::spawn`) so it never blocks the
/// response. Public routes (no valid session) are silently skipped.
pub async fn audit_layer(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // Extract the session token from the cookie header (non-blocking parse).
    // Actual session validation happens asynchronously after the response.
    let session_token = request
        .headers()
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(extract_session_token);

    let response = next.run(request).await;

    // Only log mutating requests that succeeded (2xx).
    let status = response.status().as_u16();
    let is_mutation = matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE");
    let is_success = (200..300).contains(&(status as usize));

    if is_mutation && is_success {
        if let Some(token) = session_token {
            let pool = state.pool.clone();
            let action = format!("{method} {path}");
            tokio::spawn(async move {
                resolve_and_insert_audit(&pool, &token, &action, &method, &path, status as i32, &request_id).await;
            });
        }
    }

    response
}

/// Validate the session token and insert an audit log entry.
/// Errors are logged but never propagated.
async fn resolve_and_insert_audit(
    pool: &PgPool,
    session_token: &str,
    action: &str,
    method: &str,
    path: &str,
    status_code: i32,
    request_id: &str,
) {
    let session = match crate::auth::session::validate_session(pool, session_token).await {
        Ok(s) => s,
        Err(_) => return, // Invalid/expired session — skip audit silently
    };

    insert_audit_entry(
        pool,
        session.cabinet_id,
        session.user_id,
        action,
        method,
        path,
        status_code,
        request_id,
    )
    .await;
}

/// Fire-and-forget INSERT. Errors are logged but never propagated.
async fn insert_audit_entry(
    pool: &PgPool,
    cabinet_id: Uuid,
    user_id: Uuid,
    action: &str,
    method: &str,
    path: &str,
    status_code: i32,
    request_id: &str,
) {
    // We need to set app.current_tenant for RLS on audit_log.
    let result: Result<(), sqlx::Error> = async {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(cabinet_id.to_string())
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO audit_log (cabinet_id, user_id, action, method, path, status_code, request_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(cabinet_id)
        .bind(user_id)
        .bind(action)
        .bind(method)
        .bind(path)
        .bind(status_code)
        .bind(request_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
    .await;

    if let Err(e) = result {
        tracing::warn!("Failed to insert audit log entry: {e}");
    }
}
