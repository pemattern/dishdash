use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(GenerateSchemas)]
#[schema(read_only)]
pub struct RecipeMealType {
    pub recipe_id: Uuid,
    pub meal_type_code: String,
}
