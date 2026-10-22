use crate::{error::RepositoryResult, models::recipe::Recipe};
use uuid::Uuid;

pub struct Recipes;

impl Recipes {
    pub async fn create(&self, name: String, summary: String) -> RepositoryResult<Recipe> {
        let query = "
            INSERT INTO recipes (name, summary)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let recipe = sqlx::query_as::<_, Recipe>(query)
            .bind(&name)
            .bind(&summary)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe)
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<Recipe> {
        let query = "
            UPDATE recipes
            SET
                name = COALESCE($1, name),
                summary = COALESCE($2, summary),
            WHERE id = $3
            RETURNING *;
        ";
        let recipe = sqlx::query_as::<_, Recipe>(query)
            .bind(&name)
            .bind(&summary)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe)
    }
}
