use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::auth::AuthError;

/// Hash a plaintext password using Argon2id.
///
/// Returns a PHC-formatted string containing the algorithm params, salt,
/// and hash — suitable for storage in `utilisateur.password_hash`.
pub fn hash_password(plain: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(plain.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::PasswordHash(e.to_string()))
}

/// Verify a plaintext password against a stored Argon2 hash.
///
/// Returns `Ok(false)` for wrong password, `Err` for corrupted hash data.
pub fn verify_password(plain: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AuthError::PasswordHash(e.to_string()))?;

    match Argon2::default().verify_password(plain.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(AuthError::PasswordHash(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_correct_password() {
        let plain = "S3cur3P@ssw0rd!";
        let hash = hash_password(plain).unwrap();
        assert!(verify_password(plain, &hash).unwrap());
    }

    #[test]
    fn wrong_password_returns_false() {
        let hash = hash_password("correct").unwrap();
        assert!(!verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn different_hashes_for_same_password() {
        let h1 = hash_password("same").unwrap();
        let h2 = hash_password("same").unwrap();
        // Salts differ → hashes differ
        assert_ne!(h1, h2);
        // Both verify correctly
        assert!(verify_password("same", &h1).unwrap());
        assert!(verify_password("same", &h2).unwrap());
    }

    #[test]
    fn invalid_hash_string_returns_error() {
        let result = verify_password("any", "not-a-valid-hash");
        assert!(result.is_err());
    }
}
