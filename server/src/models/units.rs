use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Unit {
    pub physical_quantity_id: Uuid,
    pub unit_system_id: Uuid,
    pub name: String,
    pub symbol: String,
    pub si_factor: f64,
}
