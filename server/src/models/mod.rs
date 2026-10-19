mod ingredient;
mod physical_quantities;
mod recipe;
mod recipe_ingredients;
mod refresh_token;
mod unit_systems;
mod units;
mod user;

pub use ingredient::Ingredient;
pub use physical_quantities::PhysicalQuantity;
pub use recipe::Recipe;
pub use recipe_ingredients::RecipeIngredient;
pub use refresh_token::RefreshToken;
pub use unit_systems::UnitSystem;
pub use units::Unit;
pub use user::User;

pub trait Model: Send + Unpin + Clone {
    const TABLE_NAME: &str;
}
