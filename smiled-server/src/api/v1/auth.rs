use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{
        jwt::{create_token_pair, validate_refresh_token},
        middleware::AuthUser,
        password::{hash_password, verify_password},
        AuthError,
    },
    state::AppState,
};

// ─── Router ──────────────────────────────────────────────────────────────────

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        .route("/me", get(me))
}

// ─── Request/Response DTOs ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserProfile,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub cabinet_id: String,
    pub role: String,
    pub nom: String,
    pub prenom: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

// ─── Handlers ────────────────────────────────────────────────────────────────

/// POST /api/v1/auth/login
///
/// Validate email + password and return a JWT access + refresh token pair.
async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthApiError> {
    // Fetch user by email (no RLS needed for login — cabinet context not yet known)
    let user = sqlx::query!(
        r#"
        SELECT id, cabinet_id, role, nom, prenom, email, password_hash, actif
        FROM utilisateur
        WHERE email = $1
        "#,
        body.email.to_lowercase(),
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AuthApiError::Database(e.to_string()))?
    .ok_or(AuthApiError::InvalidCredentials)?;

    // Reject disabled accounts
    if !user.actif.unwrap_or(true) {
        return Err(AuthApiError::AccountDisabled);
    }

    // Constant-time password check
    let valid = verify_password(&body.password, &user.password_hash)
        .map_err(|e| AuthApiError::Internal(e.to_string()))?;

    if !valid {
        return Err(AuthApiError::InvalidCredentials);
    }

    let tokens = create_token_pair(
        user.id,
        user.cabinet_id,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )
    .map_err(|e| AuthApiError::Internal(e.to_string()))?;

    Ok(Json(LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: UserProfile {
            id: user.id.to_string(),
            cabinet_id: user.cabinet_id.to_string(),
            role: user.role,
            nom: user.nom,
            prenom: user.prenom,
            email: user.email,
        },
    }))
}

/// POST /api/v1/auth/refresh
///
/// Exchange a valid refresh token for a new access + refresh token pair.
async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AuthApiError> {
    let claims = validate_refresh_token(&body.refresh_token, &state.config.jwt_secret)
        .map_err(|e| match e {
            AuthError::TokenExpired => AuthApiError::TokenExpired,
            _ => AuthApiError::InvalidToken,
        })?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AuthApiError::InvalidToken)?;
    let cabinet_id =
        Uuid::parse_str(&claims.cabinet_id).map_err(|_| AuthApiError::InvalidToken)?;

    // Verify user still exists and is active
    let active: bool = sqlx::query_scalar!(
        "SELECT actif FROM utilisateur WHERE id = $1",
        user_id,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AuthApiError::Database(e.to_string()))?
    .flatten()
    .unwrap_or(false);

    if !active {
        return Err(AuthApiError::AccountDisabled);
    }

    let tokens = create_token_pair(
        user_id,
        cabinet_id,
        &claims.role,
        &state.config.jwt_secret,
        state.config.jwt_expiry_hours,
    )
    .map_err(|e| AuthApiError::Internal(e.to_string()))?;

    Ok(Json(RefreshResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

/// GET /api/v1/me
///
/// Return the authenticated user's profile.
async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserProfile>, AuthApiError> {
    let user = sqlx::query!(
        r#"
        SELECT id, cabinet_id, role, nom, prenom, email
        FROM utilisateur
        WHERE id = $1
        "#,
        auth_user.user_id,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AuthApiError::Database(e.to_string()))?
    .ok_or(AuthApiError::NotFound)?;

    Ok(Json(UserProfile {
        id: user.id.to_string(),
        cabinet_id: user.cabinet_id.to_string(),
        role: user.role,
        nom: user.nom,
        prenom: user.prenom,
        email: user.email,
    }))
}

/// POST /api/v1/auth/forgot-password
///
/// Initiate a password reset by email.
/// Always returns 200 to prevent user enumeration.
async fn forgot_password(
    State(state): State<AppState>,
    Json(body): Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, AuthApiError> {
    let email = body.email.to_lowercase();

    // Look up user — silently succeed if not found (no enumeration)
    let user = sqlx::query!(
        "SELECT id, nom, prenom FROM utilisateur WHERE email = $1 AND actif = true",
        email,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AuthApiError::Database(e.to_string()))?;

    if let Some(user) = user {
        // Generate a reset token valid for 1 hour
        let reset_token = create_password_reset_token(user.id, &state.config.jwt_secret)?;

        // Store the hashed token in the database
        let token_hash = hash_password(&reset_token)
            .map_err(|e| AuthApiError::Internal(e.to_string()))?;

        let expires_at = Utc::now() + chrono::Duration::hours(1);

        sqlx::query!(
            r#"
            INSERT INTO password_reset_token (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE
              SET token_hash = EXCLUDED.token_hash,
                  expires_at = EXCLUDED.expires_at,
                  used = false
            "#,
            user.id,
            token_hash,
            expires_at,
        )
        .execute(&state.pool)
        .await
        .map_err(|e: sqlx::Error| AuthApiError::Database(e.to_string()))?;

        // Send email if SMTP is configured
        if state.config.smtp_host.is_some() {
            if let Err(e) = send_reset_email(&state, &email, &user.prenom, &reset_token).await {
                tracing::error!("Failed to send password reset email to {email}: {e}");
            }
        } else {
            tracing::info!(
                user_id = %user.id,
                "Password reset token generated (SMTP not configured)"
            );
            tracing::debug!(
                user_id = %user.id,
                token = %reset_token,
                "Reset token value (debug only)"
            );
        }
    }

    Ok(Json(serde_json::json!({
        "message": "Si un compte existe avec cet email, un lien de réinitialisation a été envoyé."
    })))
}

/// POST /api/v1/auth/reset-password
///
/// Apply a new password given a valid reset token.
async fn reset_password(
    State(state): State<AppState>,
    Json(body): Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, AuthApiError> {
    // Validate new password length early (before any expensive work)
    if body.new_password.len() < 8 {
        return Err(AuthApiError::ValidationError(
            "Le mot de passe doit contenir au moins 8 caractères".to_string(),
        ));
    }

    // Decode the reset token to get user_id
    let user_id = decode_password_reset_token(&body.token, &state.config.jwt_secret)
        .map_err(|_| AuthApiError::InvalidToken)?;

    // Fetch stored token record
    let record = sqlx::query!(
        r#"
        SELECT token_hash, expires_at, used
        FROM password_reset_token
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e: sqlx::Error| AuthApiError::Database(e.to_string()))?
    .ok_or(AuthApiError::InvalidToken)?;

    // Check used flag
    if record.used {
        return Err(AuthApiError::InvalidToken);
    }

    // Check expiry
    if record.expires_at < Utc::now() {
        return Err(AuthApiError::TokenExpired);
    }

    // Verify the raw token against the stored hash
    let valid = verify_password(&body.token, &record.token_hash)
        .map_err(|e| AuthApiError::Internal(e.to_string()))?;

    if !valid {
        return Err(AuthApiError::InvalidToken);
    }

    // Hash the new password
    let new_hash = hash_password(&body.new_password)
        .map_err(|e| AuthApiError::Internal(e.to_string()))?;

    // Update password and mark token as used in a single transaction
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|e| AuthApiError::Database(e.to_string()))?;

    sqlx::query!(
        "UPDATE utilisateur SET password_hash = $1 WHERE id = $2",
        new_hash,
        user_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e: sqlx::Error| AuthApiError::Database(e.to_string()))?;

    sqlx::query!(
        "UPDATE password_reset_token SET used = true WHERE user_id = $1",
        user_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e: sqlx::Error| AuthApiError::Database(e.to_string()))?;

    tx.commit()
        .await
        .map_err(|e| AuthApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "message": "Mot de passe réinitialisé avec succès."
    })))
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Create a short-lived JWT used as a password-reset token.
fn create_password_reset_token(user_id: Uuid, secret: &str) -> Result<String, AuthApiError> {
    use crate::auth::jwt::create_token;
    create_token(
        user_id,
        Uuid::nil(), // cabinet_id not needed for reset
        "reset",
        secret,
        1, // 1 hour
        "password_reset",
    )
    .map_err(|e| AuthApiError::Internal(e.to_string()))
}

/// Decode a password-reset token and return the user_id.
fn decode_password_reset_token(token: &str, secret: &str) -> Result<Uuid, AuthError> {
    use crate::auth::jwt::validate_token;
    let data = validate_token(token, secret)?;

    if data.claims.token_type != "password_reset" {
        return Err(AuthError::InvalidToken("Not a reset token".to_string()));
    }

    Uuid::parse_str(&data.claims.sub)
        .map_err(|_| AuthError::InvalidToken("Bad user_id".to_string()))
}

/// Send a password reset email using the configured SMTP server (async, non-blocking).
async fn send_reset_email(
    state: &AppState,
    to_email: &str,
    prenom: &str,
    reset_token: &str,
) -> Result<(), String> {
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    };

    let smtp_host = state.config.smtp_host.as_deref().unwrap_or("");
    let smtp_port = state.config.smtp_port.unwrap_or(587);
    let smtp_user = state.config.smtp_user.as_deref().unwrap_or("").to_string();
    let smtp_password = state.config.smtp_password.as_deref().unwrap_or("").to_string();

    let reset_url = format!("{}/reset-password?token={reset_token}", state.config.app_base_url);

    let body = format!(
        "Bonjour {prenom},\n\nVous avez demandé une réinitialisation de votre mot de passe.\n\nCliquez sur le lien suivant (valable 1 heure) :\n{reset_url}\n\nSi vous n'êtes pas à l'origine de cette demande, ignorez cet email.\n\nL'équipe Smiled.IO"
    );

    let email = Message::builder()
        .from(state.config.smtp_from.parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
        .to(to_email.parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
        .subject("Réinitialisation de votre mot de passe Smiled.IO")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(smtp_user, smtp_password);

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_host)
            .map_err(|e| e.to_string())?
            .port(smtp_port)
            .credentials(creds)
            .build();

    mailer.send(email).await.map_err(|e| e.to_string())?;

    Ok(())
}

// ─── Error Type ───────────────────────────────────────────────────────────────

#[derive(Debug)]
enum AuthApiError {
    InvalidCredentials,
    AccountDisabled,
    InvalidToken,
    TokenExpired,
    NotFound,
    ValidationError(String),
    Database(String),
    Internal(String),
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthApiError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Email ou mot de passe incorrect".to_string())
            }
            AuthApiError::AccountDisabled => {
                (StatusCode::FORBIDDEN, "Compte désactivé".to_string())
            }
            AuthApiError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Jeton invalide ou expiré".to_string())
            }
            AuthApiError::TokenExpired => {
                (StatusCode::UNAUTHORIZED, "Jeton expiré".to_string())
            }
            AuthApiError::NotFound => {
                (StatusCode::NOT_FOUND, "Utilisateur introuvable".to_string())
            }
            AuthApiError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AuthApiError::Database(e) => {
                tracing::error!("Database error in auth handler: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erreur serveur".to_string(),
                )
            }
            AuthApiError::Internal(e) => {
                tracing::error!("Internal error in auth handler: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erreur serveur".to_string(),
                )
            }
        };

        (
            status,
            Json(serde_json::json!({ "error": message })),
        )
            .into_response()
    }
}
