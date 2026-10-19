mod ingredient;
mod physical_quantities;
mod recipe;
mod recipe_ingredients;
mod refresh_token;
mod unit_systems;
mod units;
mod user;

use chrono::{DateTime, Utc};
pub use ingredient::Ingredient;
pub use physical_quantities::PhysicalQuantity;
pub use recipe::Recipe;
pub use recipe_ingredients::RecipeIngredient;
pub use refresh_token::RefreshToken;
use sqlx::prelude::FromRow;
pub use unit_systems::UnitSystem;
pub use units::Unit;
pub use user::User;
use uuid::Uuid;

pub trait Model: Send + Sync + Unpin + Clone {
    const TABLE_NAME: &str;
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct BaseModel<M>
where
    M: Model,
{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(flatten)]
    pub model: M,
}
