use lib::GenerateSchemas;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct Ingredient {
    pub code: String,
    pub ingredient_category_code: String,
}

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct IngredientSearch {
    pub code: String,
    pub ingredient_category_code: String,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
