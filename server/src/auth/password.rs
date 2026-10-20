use crate::error::ServiceResult;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::sync::LazyLock;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(Argon2::default);

pub fn is_valid_password(password: &str, password_hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(password_hash)
        && ARGON2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    {
        return true;
    }
    false
}

pub fn get_hash(password: &str) -> ServiceResult<String> {
    Ok(ARGON2
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())?)
}
