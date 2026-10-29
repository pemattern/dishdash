use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::ingredient::Ingredient;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Ingredient> for IngredientResponse {
    fn from(ingredient: Ingredient) -> Self {
        Self {
            id: ingredient.id,
            name: ingredient.name,
            created_at: ingredient.created_at,
            updated_at: ingredient.updated_at,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostIngredientRequest {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchIngredientRequest {
    pub name: Option<String>,
}
