use sqlx::prelude::FromRow;

use crate::models::Model;

#[derive(Debug, Clone, FromRow)]
pub struct PhysicalQuantity {
    pub name: String,
}

impl Model for PhysicalQuantity {
    const TABLE_NAME: &str = "physical_quantities";
}
