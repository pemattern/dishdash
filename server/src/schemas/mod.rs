pub mod recipe_ingredient;
pub mod token;
pub mod user;

use crate::error::ServiceResult;
use serde::Serialize;

fn json<T: Serialize>(val: T) -> ServiceResult<String> {
    Ok(serde_json::to_string_pretty(&val)?)
}
