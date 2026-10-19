use sqlx::prelude::FromRow;

use crate::models::Model;

#[cfg(test)]
use fake::faker::internet::en::SafeEmail;

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct User {
    #[cfg_attr(test, dummy(faker = "SafeEmail()"))]
    pub email: String,
    pub password_hash: String,
}

impl Model for User {
    const TABLE_NAME: &str = "users";
}
