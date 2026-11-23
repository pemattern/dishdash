use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::requests::constants::{UnitCode, UnitSystemCode};

pub struct RecipeIngredientModel {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub recipe_id: Uuid,
    pub ingredient_code: String,
    pub unit_code: UnitCode,
    pub unit_system_code: UnitSystemCode,
    pub position: i32,
    pub quantity: f32,
}
