use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub enum PatchField<T> {
    #[default]
    Omitted,
    Cleared,
    Value(T),
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for PatchField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(value) => PatchField::Value(value),
            None => PatchField::Cleared,
        })
    }
}
