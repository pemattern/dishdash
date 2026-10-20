use crate::models::Model;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UnitSystem {
    pub name: String,
}

impl Model for UnitSystem {
    const TABLE_NAME: &str = "unit_systems";
}
