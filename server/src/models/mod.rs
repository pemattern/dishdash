use validator::ValidationError;

use crate::requests::patch_field::PatchField;

pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod recipe_meal_type;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

pub fn validate_non_nullable_patch<T>(field: &PatchField<T>) -> Result<(), ValidationError> {
    match field {
        PatchField::Cleared => Err(ValidationError::new("cannot_clear_non_nullable_field")),
        _ => Ok(()),
    }
}
