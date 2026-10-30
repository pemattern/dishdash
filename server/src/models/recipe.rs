use chrono::{DateTime, Utc};
use lib::GenerateSchemas;
use uuid::Uuid;

#[derive(Debug, Clone, GenerateSchemas)]
pub struct Recipe {
    #[schema(read_only)]
    pub id: Uuid,
    #[schema(read_only)]
    pub created_at: DateTime<Utc>,
    #[schema(read_only)]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub summary: String,
}
