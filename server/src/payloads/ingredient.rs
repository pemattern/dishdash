use lib::Payload;

#[derive(Payload)]
#[payload(read_only)]
struct IngredientPayload {
    pub code: String,
    pub ingredient_category_code: String,
    pub name: String,
    pub name_plural: String,
    pub alias: Option<String>,
}
