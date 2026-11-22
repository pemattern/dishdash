use crate::requests::constants::IngredientCategory;

pub struct IngredientModel {
    pub code: String,
    pub ingredient_category: IngredientCategory,
}

pub struct IngredientSearch {
    pub code: String,
    pub ingredient_category: IngredientCategory,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
