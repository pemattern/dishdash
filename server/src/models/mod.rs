pub mod ingredient;
pub mod physical_quantity;
pub mod recipe;
pub mod recipe_ingredient;
pub mod refresh_token;
pub mod unit;
pub mod unit_system;
pub mod user;

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

pub trait Model: Send + Sync + Unpin + Clone + for<'a> FromRow<'a, PgRow> {
    const TABLE_NAME: &str;
}

#[derive(Debug, Clone, FromRow)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct BaseModel<M>
where
    M: Model,
{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(flatten)]
    pub model: M,
}

impl<M> BaseModel<M>
where
    M: Model,
{
    pub fn new(model: M) -> Self {
        let id = Uuid::now_v7();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        Self {
            id,
            created_at,
            updated_at,
            model,
        }
    }
}
