use chrono::{DateTime, Utc};
use lib::Model;
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct RecipeIngredient {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
    pub unit_id: Uuid,
    pub quantity: f64,
}
