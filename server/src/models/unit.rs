use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(Debug, Clone, GenerateSchemas)]
#[schema(read_only)]
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
