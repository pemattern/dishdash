use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeOwned};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct UuidMap<T>
where
    T: DeserializeOwned + Validate,
{
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: HashMap<Uuid, T>,
}

impl<T> UuidMap<T>
where
    T: DeserializeOwned + Validate,
{
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> IntoIterator for UuidMap<T>
where
    T: DeserializeOwned + Validate,
{
    type Item = (Uuid, T);
    type IntoIter = std::collections::hash_map::IntoIter<Uuid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
