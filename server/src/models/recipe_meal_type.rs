use uuid::Uuid;

use crate::requests::constants::MealType;

pub struct RecipeMealTypeModel {
    pub recipe_id: Uuid,
    pub meal_type: MealType,
}
