use chrono::{DateTime, Utc};
use lib::Payload;
use uuid::Uuid;

use crate::models::RecipeIngredientModel;

#[derive(Payload)]
struct RecipeIngredientPayload {
    #[payload(read_only)]
    pub created_at: DateTime<Utc>,
    #[payload(read_only)]
    pub updated_at: DateTime<Utc>,
    #[payload(read_only)]
    pub recipe_id: Uuid,
    #[payload(read_only)]
    pub ingredient_code: String,
    pub unit_code: String,
    pub position: i32,
    pub quantity: f32,
}

impl From<RecipeIngredientModel> for RecipeIngredientResponse {
    fn from(value: RecipeIngredientModel) -> Self {
        Self {
            created_at: value.created_at,
            updated_at: value.updated_at,
            recipe_id: value.recipe_id,
            ingredient_code: value.ingredient_code,
            unit_code: value.unit_code,
            position: value.position,
            quantity: value.quantity,
        }
    }
}
