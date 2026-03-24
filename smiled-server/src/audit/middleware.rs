use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::state::AppState;

/// Tower middleware that logs mutating requests (POST/PUT/PATCH/DELETE) to the
/// `audit_log` table after the response has been produced.
///
/// The INSERT is fire-and-forget (`tokio::spawn`) so it never blocks the
/// response. Public routes (no valid JWT) are silently skipped.
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

    // Try to extract auth info from the request extensions (set by AuthUser extractor
    // in upstream handlers). At this layer we cannot run the extractor ourselves without
    // consuming the body, so we parse the JWT manually from headers/cookies.
    let auth_info = extract_auth_info(&request, &state);

    let response = next.run(request).await;

    // Only log mutating requests that succeeded (2xx).
    let status = response.status().as_u16();
    let is_mutation = matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE");
    let is_success = (200..300).contains(&(status as usize));

    if is_mutation && is_success {
        if let Some((user_id, cabinet_id)) = auth_info {
            let pool = state.pool.clone();
            let action = format!("{method} {path}");
            tokio::spawn(async move {
                insert_audit_entry(
                    &pool,
                    cabinet_id,
                    user_id,
                    &action,
                    &method,
                    &path,
                    status as i32,
                    &request_id,
                )
                .await;
            });
        }
    }

    response
}

/// Parse JWT from cookie or Authorization header without consuming the request.
fn extract_auth_info(request: &Request<Body>, state: &AppState) -> Option<(Uuid, Uuid)> {
    let token = extract_token_from_cookie(request)
        .or_else(|| extract_token_from_header(request))?;

    let token_data =
        crate::auth::jwt::validate_token(&token, &state.config.jwt_secret).ok()?;

    if token_data.claims.token_type != "access" {
        return None;
    }

    let user_id = Uuid::parse_str(&token_data.claims.sub).ok()?;
    let cabinet_id = Uuid::parse_str(&token_data.claims.cabinet_id).ok()?;

    Some((user_id, cabinet_id))
}

fn extract_token_from_cookie(request: &Request<Body>) -> Option<String> {
    let cookie_header = request.headers().get(axum::http::header::COOKIE)?;
    let cookie_str = cookie_header.to_str().ok()?;
    cookie_str.split(';').find_map(|pair| {
        let pair = pair.trim();
        let (name, value) = pair.split_once('=')?;
        if name.trim() == "access_token" {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

fn extract_token_from_header(request: &Request<Body>) -> Option<String> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    auth_header.strip_prefix("Bearer ").map(|t| t.to_string())
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
