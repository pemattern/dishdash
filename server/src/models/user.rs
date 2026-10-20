use crate::models::Model;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            email,
            password_hash,
        }
    }
}

impl Model for User {
    const TABLE_NAME: &str = "users";
}
