use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RecipeIngredientModel {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub recipe_id: Uuid,
    pub ingredient_code: String,
    pub unit_code: String,
    pub position: i32,
    pub quantity: f32,
}
