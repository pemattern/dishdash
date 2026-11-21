use lib::Payload;

use crate::models::IngredientSearch;

#[derive(Payload)]
#[payload(read_only)]
struct IngredientPayload {
    pub code: String,
    pub ingredient_category_code: String,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}

impl From<IngredientSearch> for IngredientResponse {
    fn from(value: IngredientSearch) -> Self {
        Self {
            code: value.code,
            ingredient_category_code: value.ingredient_category_code,
            name: value.name,
            name_plural: value.name_plural,
            alias: value.alias,
        }
    }
}
