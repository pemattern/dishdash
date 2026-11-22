use std::sync::Arc;

use sqlx::error::DatabaseError;

use crate::error::AppError;

mod ingredients;
mod recipe_ingredients;
mod recipe_meal_types;
mod recipes;
mod refresh_tokens;
mod units;
mod users;

pub use ingredients::Ingredients;
pub use recipe_ingredients::RecipeIngredients;
pub use recipe_meal_types::RecipeMealTypes;
pub use recipes::Recipes;
pub use refresh_tokens::RefreshTokens;
pub use units::Units;
pub use users::Users;

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(error) => handle_sqlx_database_error(error),
            sqlx::Error::RowNotFound => AppError::ResourceNotFound,
            _ => AppError::Internal(Arc::new(error)),
        }
    }
}

fn handle_sqlx_database_error(error: Box<dyn DatabaseError + 'static>) -> AppError {
    use sqlx::error::ErrorKind;
    match error.kind() {
        ErrorKind::UniqueViolation => AppError::AlreadyExists,
        ErrorKind::ForeignKeyViolation
        | ErrorKind::ExclusionViolation
        | ErrorKind::NotNullViolation
        | ErrorKind::CheckViolation => AppError::InvalidData,
        _ => AppError::Internal(Arc::new(error)),
    }
}
