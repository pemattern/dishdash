use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PhysicalQuantity {
    pub name: String,
}
