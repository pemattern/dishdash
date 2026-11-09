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
    pub ingredient_id: Uuid,
    pub unit_id: Uuid,
    pub position: i32,
    pub quantity: f32,
}

pub mod join {}
