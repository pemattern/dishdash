use uuid::Uuid;

pub struct RecipeMealTypeModel {
    pub recipe_id: Uuid,
    pub meal_type_code: String,
}
