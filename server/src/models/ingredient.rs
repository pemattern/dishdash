use crate::requests::constants::IngredientCategoryCode;

pub struct IngredientModel {
    pub code: String,
    pub ingredient_category_code: IngredientCategoryCode,
}

pub struct IngredientSearch {
    pub code: String,
    pub ingredient_category_code: IngredientCategoryCode,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
