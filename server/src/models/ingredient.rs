use crate::models::Model;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Ingredient {
    pub name: String,
}

impl Model for Ingredient {
    const TABLE_NAME: &str = "ingredients";
}
