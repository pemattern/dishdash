use std::sync::Arc;

use sqlx::error::DatabaseError;

use crate::error::AppError;

pub mod ingredients;
pub mod physical_quantities;
pub mod recipe_ingredients;
pub mod recipes;
pub mod refresh_tokens;
pub mod unit_systems;
pub mod units;
pub mod users;

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
