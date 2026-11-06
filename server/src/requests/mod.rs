pub mod app_json;
pub mod auth;
pub mod uuid_path;

use std::collections::HashMap;

use uuid::Uuid;

pub type UuidMap<T> = HashMap<Uuid, T>;
