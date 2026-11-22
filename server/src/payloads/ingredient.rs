use lib::Payload;

use crate::{models::IngredientSearch, requests::constants::IngredientCategory};

#[allow(dead_code)]
#[derive(Payload)]
#[payload(read_only)]
struct IngredientPayload {
    pub code: String,
    pub ingredient_category: IngredientCategory,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}

impl From<IngredientSearch> for IngredientResponse {
    fn from(value: IngredientSearch) -> Self {
        Self {
            code: value.code,
            ingredient_category: value.ingredient_category,
            name: value.name,
            name_plural: value.name_plural,
            alias: value.alias,
        }
    }
}
