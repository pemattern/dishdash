use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RecipeIngredient {
    pub recipe_id: Uuid,
    pub ingredient_it: Uuid,
    pub unit: Uuid,
    pub quantity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
