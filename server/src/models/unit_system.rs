use crate::models::Model;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct UnitSystem {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

impl Model for UnitSystem {
    const TABLE_NAME: &str = "unit_systems";
}
