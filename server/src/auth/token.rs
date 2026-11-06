use std::{sync::LazyLock, time::Duration};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::refresh_token::RefreshToken,
};

const ISSUER: &str = "dishdash";
const ACCESS_TOKEN_VALIDITY_DURATION: Duration = Duration::from_mins(15);
const KEY_LENGTH: usize = 32;
static ENCRYPTION_KEY: LazyLock<[u8; KEY_LENGTH]> = LazyLock::new(generate_byte_key::<KEY_LENGTH>);

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let access_token_expires_at = now + ACCESS_TOKEN_VALIDITY_DURATION;
        let sub = user_id;
        let exp = access_token_expires_at.timestamp() as usize;
        let iat = now.timestamp() as usize;
        let iss = String::from(ISSUER);
        Self { sub, iss, exp, iat }
    }
}

fn generate_byte_key<const N: usize>() -> [u8; N] {
    let mut key = [0u8; N];
    getrandom::fill(&mut key).unwrap();
    key
}

pub fn generate_access_token(user_id: Uuid) -> AppResult<String> {
    let claims = Claims::new(user_id);
    let key = EncodingKey::from_secret(&*ENCRYPTION_KEY);
    Ok(encode(&Header::default(), &claims, &key)?)
}

pub fn generate_refresh_token() -> String {
    let opaque_token_bytes = generate_byte_key::<KEY_LENGTH>();
    URL_SAFE_NO_PAD.encode(opaque_token_bytes)
}

pub fn get_valid_access_token_claims(access_token: &str) -> AppResult<Claims> {
    let key = DecodingKey::from_secret(&*ENCRYPTION_KEY);
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    let token_data = decode::<Claims>(access_token, &key, &validation)?;
    Ok(token_data.claims)
}

pub fn parse_refresh_token(composite_refresh_token: &str) -> AppResult<(Uuid, String)> {
    let (refresh_token_id_str, opaque_token) = composite_refresh_token
        .split_once('.')
        .ok_or(AppError::InvalidRefreshToken)?;
    let refresh_token_id =
        Uuid::parse_str(refresh_token_id_str).map_err(|_| AppError::InvalidRefreshToken)?;
    Ok((refresh_token_id, opaque_token.into()))
}

pub fn validate_refresh_token_expiry(refresh_token: &RefreshToken) -> AppResult<()> {
    if Utc::now() > refresh_token.expires_at || refresh_token.is_used {
        return Err(AppError::ExpiredRefreshToken);
    }
    Ok(())
}
