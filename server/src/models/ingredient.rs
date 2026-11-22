pub struct IngredientModel {
    pub code: String,
    pub ingredient_category: String,
}

pub struct IngredientSearch {
    pub code: String,
    pub ingredient_category: String,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
