use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct RecipeIngredientJoinIngredient {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ingredient_name: String,
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
    pub unit_id: Uuid,
    pub position: i32,
    pub quantity: f32,
}
