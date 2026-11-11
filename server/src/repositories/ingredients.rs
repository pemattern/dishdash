use sqlx::PgExecutor;

use crate::{error::AppResult, models::ingredient::Ingredient};

pub struct Ingredients;

impl Ingredients {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<Ingredient>> {
        let unit_systems = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients;")
            .fetch_all(executor)
            .await?;
        Ok(unit_systems)
    }
}
