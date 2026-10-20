pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

use sqlx::{postgres::PgRow, prelude::FromRow};

pub trait Model: Send + Sync + Unpin + Clone + for<'a> FromRow<'a, PgRow> {
    const TABLE_NAME: &str;
}
