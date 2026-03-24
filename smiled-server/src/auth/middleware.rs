use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{auth::session::validate_session, state::AppState};

/// Authenticated user extracted from the Better Auth session cookie.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub cabinet_id: Uuid,
    pub role: String,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get(axum::http::header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthRejection::MissingSession)?;

        let session_token = extract_session_token(cookie_header)
            .ok_or(AuthRejection::MissingSession)?;

        let session_user = validate_session(&state.pool, &session_token)
            .await
            .map_err(|_| AuthRejection::InvalidSession)?;

        Ok(AuthUser {
            user_id: session_user.user_id,
            cabinet_id: session_user.cabinet_id,
            role: session_user.role,
        })
    }
}

/// Extract the Better Auth session token from a cookie header string.
pub fn extract_session_token(cookie_header: &str) -> Option<String> {
    cookie_header
        .split(';')
        .find_map(|c| c.trim().strip_prefix("better-auth.session_token="))
        .map(|v| v.to_string())
}

/// Rejection type returned when session extraction fails.
#[derive(Debug)]
pub enum AuthRejection {
    MissingSession,
    InvalidSession,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthRejection::MissingSession => {
                (StatusCode::UNAUTHORIZED, "Session cookie missing")
            }
            AuthRejection::InvalidSession => {
                (StatusCode::UNAUTHORIZED, "Invalid or expired session")
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
