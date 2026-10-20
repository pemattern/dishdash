use crate::models::Model;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Recipe {
    pub name: String,
    pub summary: String,
}

impl Model for Recipe {
    const TABLE_NAME: &str = "recipes";
}
