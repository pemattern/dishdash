use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(Debug, Clone, GenerateSchemas)]
pub struct RecipeIngredient {
    #[schema(read_only)]
    pub id: Uuid,
    #[schema(read_only)]
    pub created_at: DateTime<Utc>,
    #[schema(read_only)]
    pub updated_at: DateTime<Utc>,
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
    pub unit_id: Uuid,
    pub quantity: f64,
}

pub mod join {
    use super::*;

    #[derive(Debug, Clone, GenerateSchemas)]
    #[schema(read_only)]
    pub struct RecipeIngredientJoinIngredient {
        pub id: Uuid,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
        pub ingredient_name: String,
        pub recipe_id: Uuid,
        pub ingredient_id: Uuid,
        pub unit_id: Uuid,
        pub quantity: f64,
    }
}
