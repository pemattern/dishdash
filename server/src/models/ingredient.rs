pub struct IngredientModel {
    pub code: String,
    pub ingredient_category_code: String,
}

pub struct IngredientSearch {
    pub code: String,
    pub ingredient_category_code: String,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
