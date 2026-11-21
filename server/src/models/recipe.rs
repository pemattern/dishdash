use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RecipeModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub image_url: Option<String>,
    pub duration_minutes: i32,
}
