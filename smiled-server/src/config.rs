use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub tenant_mode: TenantMode,
    pub default_cabinet_id: Option<String>,
    pub cors_origins: Vec<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_user: Option<String>,
    pub smtp_password: Option<String>,
    pub server_port: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TenantMode {
    Single,
    Multi,
}

impl TenantMode {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "single" => TenantMode::Single,
            _ => TenantMode::Multi,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required env var: {0}")]
    MissingVar(String),

    #[error("Invalid value for {var}: {reason}")]
    InvalidValue { var: String, reason: String },
}

impl Config {
    /// Load configuration from environment variables.
    /// Calls `dotenvy::dotenv()` first so `.env` files are respected.
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env if present — ignore errors (file may not exist in prod)
        let _ = dotenvy::dotenv();

        let database_url = require_var("DATABASE_URL")?;
        let jwt_secret = require_var("JWT_SECRET")?;

        let jwt_expiry_hours = optional_var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|| "24".to_string())
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidValue {
                var: "JWT_EXPIRY_HOURS".to_string(),
                reason: "must be a positive integer".to_string(),
            })?;

        let s3_endpoint = optional_var("S3_ENDPOINT")
            .unwrap_or_else(|| "http://localhost:9000".to_string());
        let s3_bucket = optional_var("S3_BUCKET")
            .unwrap_or_else(|| "smiled-docs".to_string());
        let s3_access_key = optional_var("S3_ACCESS_KEY")
            .unwrap_or_else(|| "minioadmin".to_string());
        let s3_secret_key = optional_var("S3_SECRET_KEY")
            .unwrap_or_else(|| "minioadmin".to_string());

        let tenant_mode = TenantMode::from_str(
            &optional_var("TENANT_MODE").unwrap_or_else(|| "multi".to_string()),
        );

        let default_cabinet_id = optional_var("DEFAULT_CABINET_ID");

        // Validate single-tenant requirement
        if tenant_mode == TenantMode::Single && default_cabinet_id.is_none() {
            return Err(ConfigError::MissingVar(
                "DEFAULT_CABINET_ID (required when TENANT_MODE=single)".to_string(),
            ));
        }

        let cors_origins = optional_var("CORS_ORIGINS")
            .unwrap_or_else(|| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let smtp_host = optional_var("SMTP_HOST");
        let smtp_port = optional_var("SMTP_PORT")
            .map(|v| {
                v.parse::<u16>().map_err(|_| ConfigError::InvalidValue {
                    var: "SMTP_PORT".to_string(),
                    reason: "must be a valid port number".to_string(),
                })
            })
            .transpose()?;
        let smtp_user = optional_var("SMTP_USER");
        let smtp_password = optional_var("SMTP_PASSWORD");

        let server_port = optional_var("SERVER_PORT")
            .unwrap_or_else(|| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidValue {
                var: "SERVER_PORT".to_string(),
                reason: "must be a valid port number".to_string(),
            })?;

        Ok(Config {
            database_url,
            jwt_secret,
            jwt_expiry_hours,
            s3_endpoint,
            s3_bucket,
            s3_access_key,
            s3_secret_key,
            tenant_mode,
            default_cabinet_id,
            cors_origins,
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_password,
            server_port,
        })
    }
}

fn require_var(name: &str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVar(name.to_string()))
}

fn optional_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tenant_mode_parses_correctly() {
        assert_eq!(TenantMode::from_str("single"), TenantMode::Single);
        assert_eq!(TenantMode::from_str("SINGLE"), TenantMode::Single);
        assert_eq!(TenantMode::from_str("multi"), TenantMode::Multi);
        assert_eq!(TenantMode::from_str("anything_else"), TenantMode::Multi);
    }
}
