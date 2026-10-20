use crate::models::Model;
use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Unit {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub physical_quantity_id: Uuid,
    pub unit_system_id: Uuid,
    pub name: String,
    pub symbol: String,
    pub si_factor: f64,
}

impl Model for Unit {
    const TABLE_NAME: &str = "units";
}
