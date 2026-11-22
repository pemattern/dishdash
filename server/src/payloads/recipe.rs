use chrono::{DateTime, Utc};
use lib::Payload;
use uuid::Uuid;

use crate::models::{RecipeMealTypeModel, RecipeModel};

#[allow(dead_code)]
#[derive(Payload)]
struct RecipePayload {
    #[payload(read_only)]
    pub id: Uuid,
    #[payload(read_only)]
    pub created_at: DateTime<Utc>,
    #[payload(read_only)]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[payload(write_rename = "image_data")]
    pub image_url: Option<String>,
    pub duration_minutes: i32,
    pub meal_types: Vec<String>,
}

impl From<(RecipeModel, Vec<RecipeMealTypeModel>)> for RecipeResponse {
    fn from(value: (RecipeModel, Vec<RecipeMealTypeModel>)) -> Self {
        let meal_types = value.1.into_iter().map(|mt| mt.meal_type_code).collect();
        Self {
            id: value.0.id,
            created_at: value.0.created_at,
            updated_at: value.0.updated_at,
            name: value.0.name,
            image_url: value.0.image_url,
            duration_minutes: value.0.duration_minutes,
            meal_types,
        }
    }
}
