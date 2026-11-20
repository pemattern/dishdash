use serde::{Deserialize, Deserializer};

pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod recipe_meal_type;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

fn deserialize_double_option<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
