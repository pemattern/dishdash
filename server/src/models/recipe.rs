use crate::models::Model;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub summary: String,
}

impl Model for Recipe {
    const TABLE_NAME: &str = "recipes";
}
