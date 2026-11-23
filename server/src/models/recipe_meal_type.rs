use uuid::Uuid;

use crate::requests::constants::MealTypeCode;

pub struct RecipeMealTypeModel {
    pub recipe_id: Uuid,
    pub meal_type_code: MealTypeCode,
}
