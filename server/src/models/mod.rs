pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod recipe_meal_type;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

pub fn deserialize_double_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    serde::Deserialize::deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use super::deserialize_double_option;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Patch {
        #[serde(default, deserialize_with = "deserialize_double_option")]
        name: Option<Option<String>>,

        #[serde(default)]
        age: Option<u32>,
    }

    #[test]
    fn test_patch_omitted_fields() {
        let patch: Patch = serde_json::from_str("{}").unwrap();

        assert_eq!(patch.name, None);
        assert_eq!(patch.age, None);
    }

    #[test]
    fn test_patch_set_fields() {
        let patch: Patch = serde_json::from_str(r#"{"name": "Alice", "age": 30}"#).unwrap();

        assert_eq!(patch.name, Some(Some("Alice".to_string())));
        assert_eq!(patch.age, Some(30));
    }

    #[test]
    fn test_patch_mixed_fields() {
        let patch: Patch = serde_json::from_str(r#"{"name": null, "age": 30}"#).unwrap();

        assert_eq!(patch.name, Some(None));
        assert_eq!(patch.age, Some(30));
    }
}
