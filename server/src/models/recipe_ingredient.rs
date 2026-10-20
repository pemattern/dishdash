use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{
    models::{Model, Patch},
    schemas::recipe_ingredient::PatchRecipeIngredientRequest,
};

#[derive(Debug, Clone, FromRow)]
pub struct RecipeIngredient {
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
    pub unit_id: Uuid,
    pub quantity: f64,
}

impl Model for RecipeIngredient {
    const TABLE_NAME: &str = "recipe_ingredients";
}

impl Patch for RecipeIngredient {
    type PatchSchema = PatchRecipeIngredientRequest;
}
