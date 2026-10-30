use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(Debug, Clone, GenerateSchemas)]
pub struct Unit {
    #[schema(read_only)]
    pub id: Uuid,
    #[schema(read_only)]
    pub created_at: DateTime<Utc>,
    #[schema(read_only)]
    pub updated_at: DateTime<Utc>,
    pub physical_quantity_id: Uuid,
    pub unit_system_id: Uuid,
    pub name: String,
    pub symbol: String,
    pub si_factor: f64,
}
