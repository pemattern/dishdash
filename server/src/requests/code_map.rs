use std::collections::HashMap;

use serde::{Deserialize, de::DeserializeOwned};
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct CodeMap<T>
where
    T: DeserializeOwned + Validate,
{
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: HashMap<String, T>,
}

impl<T> CodeMap<T>
where
    T: DeserializeOwned + Validate,
{
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> IntoIterator for CodeMap<T>
where
    T: DeserializeOwned + Validate,
{
    type Item = (String, T);
    type IntoIter = std::collections::hash_map::IntoIter<String, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
