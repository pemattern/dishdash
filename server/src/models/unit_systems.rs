use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UnitSystem {
    pub name: String,
}
