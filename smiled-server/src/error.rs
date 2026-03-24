use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Unified API error type for all handler modules.
///
/// Maps each variant to the correct HTTP status code and returns
/// `{"success": false, "error": "message"}`.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Ressource introuvable")]
    NotFound,

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("Action non autorisée")]
    Forbidden,

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    Storage(String),

    #[error("{0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Ressource introuvable".to_string()),
            ApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Action non autorisée".to_string()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Database(ref e) => {
                tracing::error!("Database error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
            ApiError::Storage(ref e) => {
                tracing::error!("Storage error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
            ApiError::Internal(ref e) => {
                tracing::error!("Internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
        };

        (
            status,
            Json(serde_json::json!({ "success": false, "error": message })),
        )
            .into_response()
    }
}
