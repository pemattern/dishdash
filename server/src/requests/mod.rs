pub mod app_json;
pub mod app_path;
pub mod app_typed_header;

use std::collections::HashMap;

use uuid::Uuid;

pub type UuidMap<T> = HashMap<Uuid, T>;
