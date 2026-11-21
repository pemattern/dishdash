use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub password_hash: String,
}
