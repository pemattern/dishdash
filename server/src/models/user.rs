use crate::models::Model;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub password_hash: String,
}

impl Model for User {
    const TABLE_NAME: &str = "users";
}
