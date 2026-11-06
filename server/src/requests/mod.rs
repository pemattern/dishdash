pub mod app_json;
pub mod auth;
pub mod uuid_path;

use std::collections::HashMap;

use uuid::Uuid;
use validator::Validate;

// pub type UuidMap<T> = HashMap<Uuid, T>;

#[derive(Validate)]
pub struct UuidMap<T>
where
    T: Validate,
{
    #[validate(nested)]
    map: HashMap<Uuid, T>,
}
