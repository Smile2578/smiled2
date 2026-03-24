pub mod middleware;
pub mod permissions;
pub mod session;

#[allow(unused_imports)]
pub use middleware::AuthUser;

/// Unified auth error type used across this module.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid or expired session")]
    InvalidSession,

    #[error("Database error: {0}")]
    Database(String),

    #[error("User not found")]
    UserNotFound,
}
