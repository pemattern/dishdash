use sqlx::prelude::FromRow;

use crate::models::Model;

#[derive(Debug, Clone, FromRow)]
pub struct UnitSystem {
    pub name: String,
}

impl Model for UnitSystem {
    const TABLE_NAME: &str = "unit_systems";
}
