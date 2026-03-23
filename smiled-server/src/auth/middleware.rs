use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{auth::jwt::validate_token, state::AppState};

/// Authenticated user extracted from the JWT in the `Authorization` header.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub cabinet_id: Uuid,
    pub role: String,
}

/// Roles allowed to manage reference data (actes, matériaux, teintes, etc.)
const SETTINGS_ROLES: &[&str] = &["titulaire", "associe", "admin"];

/// Roles allowed to write clinical data (schema, diagnostic, PDT, etc.)
const CLINICAL_WRITE_ROLES: &[&str] = &[
    "titulaire", "associe", "collaborateur", "remplacant",
    "specialiste_odf", "specialiste_co", "specialiste_mbd",
];

impl AuthUser {
    /// Check if the user has a role that can manage cabinet settings and reference data.
    pub fn can_manage_settings(&self) -> bool {
        SETTINGS_ROLES.contains(&self.role.as_str())
    }

    /// Check if the user has a role that can write clinical data.
    pub fn can_write_clinical(&self) -> bool {
        CLINICAL_WRITE_ROLES.contains(&self.role.as_str())
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthRejection::MissingToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthRejection::MalformedToken)?;

        let token_data =
            validate_token(token, &state.config.jwt_secret).map_err(|e| {
                use crate::auth::AuthError;
                match e {
                    AuthError::TokenExpired => AuthRejection::ExpiredToken,
                    _ => AuthRejection::InvalidToken,
                }
            })?;

        // Only accept access tokens here
        if token_data.claims.token_type != "access" {
            return Err(AuthRejection::InvalidToken);
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AuthRejection::InvalidToken)?;

        let cabinet_id = Uuid::parse_str(&token_data.claims.cabinet_id)
            .map_err(|_| AuthRejection::InvalidToken)?;

        Ok(AuthUser {
            user_id,
            cabinet_id,
            role: token_data.claims.role,
        })
    }
}

/// Rejection type returned when JWT extraction fails.
#[derive(Debug)]
pub enum AuthRejection {
    MissingToken,
    MalformedToken,
    InvalidToken,
    ExpiredToken,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthRejection::MissingToken => {
                (StatusCode::UNAUTHORIZED, "Authorization header missing")
            }
            AuthRejection::MalformedToken => {
                (StatusCode::UNAUTHORIZED, "Authorization header must be 'Bearer <token>'")
            }
            AuthRejection::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AuthRejection::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
