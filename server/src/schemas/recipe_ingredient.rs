use uuid::Uuid;

pub struct PatchRecipeIngredientRequest {
    pub recipe_id: Option<Uuid>,
    pub ingredient_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub quantity: Option<f64>,
}
