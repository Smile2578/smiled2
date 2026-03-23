use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::AuthError;

/// JWT claims embedded in every token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User UUID (subject)
    pub sub: String,
    /// Tenant (cabinet) UUID
    pub cabinet_id: String,
    /// Role code
    pub role: String,
    /// Expiry timestamp (Unix seconds)
    pub exp: usize,
    /// Issued-at timestamp (Unix seconds)
    pub iat: usize,
    /// "access" | "refresh"
    pub token_type: String,
}

/// Token pair returned on successful authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// Create an access + refresh token pair.
pub fn create_token_pair(
    user_id: Uuid,
    cabinet_id: Uuid,
    role: &str,
    jwt_secret: &str,
    access_expiry_hours: u64,
) -> Result<TokenPair, AuthError> {
    let access_token = create_token(
        user_id,
        cabinet_id,
        role,
        jwt_secret,
        access_expiry_hours,
        "access",
    )?;

    // Refresh tokens live for 30 days
    let refresh_token = create_token(
        user_id,
        cabinet_id,
        role,
        jwt_secret,
        24 * 30,
        "refresh",
    )?;

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

/// Create a single signed JWT.
pub fn create_token(
    user_id: Uuid,
    cabinet_id: Uuid,
    role: &str,
    jwt_secret: &str,
    expiry_hours: u64,
    token_type: &str,
) -> Result<String, AuthError> {
    let now = Utc::now().timestamp() as usize;
    let exp = now + (expiry_hours as usize * 3600);

    let claims = Claims {
        sub: user_id.to_string(),
        cabinet_id: cabinet_id.to_string(),
        role: role.to_string(),
        exp,
        iat: now,
        token_type: token_type.to_string(),
    };

    let key = EncodingKey::from_secret(jwt_secret.as_bytes());

    encode(&Header::default(), &claims, &key)
        .map_err(|e| AuthError::TokenCreation(e.to_string()))
}

/// Validate a JWT and return the decoded claims.
pub fn validate_token(token: &str, jwt_secret: &str) -> Result<TokenData<Claims>, AuthError> {
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = true;

    decode::<Claims>(token, &key, &validation)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken(e.to_string()),
        })
}

/// Validate a refresh token specifically.
pub fn validate_refresh_token(
    token: &str,
    jwt_secret: &str,
) -> Result<Claims, AuthError> {
    let data = validate_token(token, jwt_secret)?;

    if data.claims.token_type != "refresh" {
        return Err(AuthError::InvalidToken(
            "Expected refresh token".to_string(),
        ));
    }

    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test-secret-minimum-32-chars-long!!";
    const USER_ID: Uuid = Uuid::nil();
    const CABINET_ID: Uuid = Uuid::nil();

    #[test]
    fn create_and_validate_access_token() {
        let pair =
            create_token_pair(USER_ID, CABINET_ID, "titulaire", SECRET, 24).unwrap();

        let data = validate_token(&pair.access_token, SECRET).unwrap();
        assert_eq!(data.claims.token_type, "access");
        assert_eq!(data.claims.role, "titulaire");
    }

    #[test]
    fn create_and_validate_refresh_token() {
        let pair =
            create_token_pair(USER_ID, CABINET_ID, "titulaire", SECRET, 24).unwrap();

        let claims = validate_refresh_token(&pair.refresh_token, SECRET).unwrap();
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn access_token_rejected_as_refresh() {
        let pair =
            create_token_pair(USER_ID, CABINET_ID, "titulaire", SECRET, 24).unwrap();

        let result = validate_refresh_token(&pair.access_token, SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_secret_rejected() {
        let pair =
            create_token_pair(USER_ID, CABINET_ID, "titulaire", SECRET, 24).unwrap();

        let result = validate_token(&pair.access_token, "wrong-secret");
        assert!(result.is_err());
    }

    #[test]
    fn expired_token_rejected() {
        // Create a token with exp in the past by using a negative offset
        let now = Utc::now().timestamp() as usize;
        let expired_claims = Claims {
            sub: USER_ID.to_string(),
            cabinet_id: CABINET_ID.to_string(),
            role: "titulaire".to_string(),
            exp: now - 3600, // 1 hour ago
            iat: now - 7200,
            token_type: "access".to_string(),
        };

        let key = EncodingKey::from_secret(SECRET.as_bytes());
        let token = encode(&Header::default(), &expired_claims, &key).unwrap();

        let result = validate_token(&token, SECRET);
        assert!(result.is_err());
    }
}
