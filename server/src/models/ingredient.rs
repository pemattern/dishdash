use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Ingredient {
    pub name: String,
}
