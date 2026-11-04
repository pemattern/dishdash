pub mod valid_json;
pub mod valid_path;

use std::collections::HashMap;

use uuid::Uuid;

pub type UuidMap<T> = HashMap<Uuid, T>;
