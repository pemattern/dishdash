use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub opaque_token_hash: String,
    pub is_used: bool,
    pub expires_at: DateTime<Utc>,
}
