use lib::GenerateSchemas;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct Ingredient {
    pub code: String,
    pub ingredient_category_code: String,
}
