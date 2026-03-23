pub mod jwt;
pub mod middleware;
pub mod password;
pub mod permissions;

#[allow(unused_imports)]
pub use middleware::AuthUser;

/// Unified auth error type used across this module.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Token creation failed: {0}")]
    TokenCreation(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token has expired")]
    TokenExpired,

    #[error("Password hashing error: {0}")]
    PasswordHash(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,
}
