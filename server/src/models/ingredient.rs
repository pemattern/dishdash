use sqlx::prelude::FromRow;

use crate::models::Model;

#[derive(Debug, Clone, FromRow)]
pub struct Ingredient {
    pub name: String,
}

impl Model for Ingredient {
    const TABLE_NAME: &str = "ingredients";
}
