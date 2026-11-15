use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(GenerateSchemas)]
pub struct RecipeIngredient {
    #[schema(read_only)]
    pub created_at: DateTime<Utc>,
    #[schema(read_only)]
    pub updated_at: DateTime<Utc>,
    #[schema(read_only)]
    pub recipe_id: Uuid,
    #[schema(read_only)]
    pub ingredient_code: String,
    pub unit_code: String,
    pub position: i32,
    pub quantity: f32,
}

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct RecipeIngredientJoinIngredient {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub ingredient_name: String,
    pub recipe_id: Uuid,
    pub ingredient_code: String,
    pub unit_code: String,
    pub position: i32,
    pub quantity: f32,
}
