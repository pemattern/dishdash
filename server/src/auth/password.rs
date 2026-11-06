use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::sync::LazyLock;

use crate::error::AppResult;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(Argon2::default);

pub fn verify_password(password: &str, password_hash: &str) -> AppResult<()> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    ARGON2.verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(())
}

pub fn get_hash(password: &str) -> AppResult<String> {
    Ok(ARGON2
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())?)
}
