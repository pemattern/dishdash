use std::{sync::LazyLock, time::Duration};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ServiceResult;

const ISSUER: &str = "dishdash";
const ACCESS_TOKEN_VALIDITY_DURATION: Duration = Duration::from_mins(15);
const KEY_LENGTH: usize = 32;
static ENCRYPTION_KEY: LazyLock<[u8; KEY_LENGTH]> = LazyLock::new(generate_byte_key::<KEY_LENGTH>);

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let access_token_expires_at = now + ACCESS_TOKEN_VALIDITY_DURATION;
        let sub = user_id.to_string();
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

pub fn generate_access_token(user_id: Uuid) -> ServiceResult<String> {
    let claims = Claims::new(user_id);
    let key = EncodingKey::from_secret(&*ENCRYPTION_KEY);
    Ok(encode(&Header::default(), &claims, &key)?)
}

pub fn generate_refresh_token() -> String {
    let opaque_token_bytes = generate_byte_key::<KEY_LENGTH>();
    URL_SAFE_NO_PAD.encode(opaque_token_bytes)
}

pub fn get_valid_access_token_claims(access_token: &str) -> Option<Claims> {
    let key = DecodingKey::from_secret(&*ENCRYPTION_KEY);
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    decode::<Claims>(access_token, &key, &validation)
        .ok()
        .map(|token_data| token_data.claims)
}
