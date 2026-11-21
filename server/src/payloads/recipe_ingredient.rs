use chrono::{DateTime, Utc};
use lib::Payload;
use uuid::Uuid;

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
