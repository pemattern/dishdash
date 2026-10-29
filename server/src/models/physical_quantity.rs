use chrono::{DateTime, Utc};
use lib::Model;
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct PhysicalQuantity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}
