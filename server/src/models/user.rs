use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
}
