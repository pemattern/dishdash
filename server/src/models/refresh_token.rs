use crate::models::Model;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::time::Duration;
use uuid::Uuid;

const REFRESH_TOKEN_VALIDITY_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 60);

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub opaque_token_hash: String,
    pub is_used: bool,
    pub expires_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, parent_id: Option<Uuid>, opaque_token_hash: String) -> Self {
        let is_used = false;
        let expires_at = Utc::now() + REFRESH_TOKEN_VALIDITY_DURATION;
        Self {
            user_id,
            parent_id,
            opaque_token_hash,
            is_used,
            expires_at,
        }
    }
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used
    }
}

impl Model for RefreshToken {
    const TABLE_NAME: &str = "refresh_tokens";
}
