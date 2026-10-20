use crate::{
    error::RepositoryResult,
    models::{BaseModel, recipe::Recipe},
    repositories::{PostgresRepository, Repository},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RecipeRepository: Repository<Recipe> {
    async fn create(&self, name: String, summary: String) -> RepositoryResult<BaseModel<Recipe>>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<BaseModel<Recipe>>;
}

#[async_trait]
impl RecipeRepository for PostgresRepository<Recipe> {
    async fn create(&self, name: String, summary: String) -> RepositoryResult<BaseModel<Recipe>> {
        let query = "
            INSERT INTO recipes (name, summary)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let recipe = sqlx::query_as::<_, BaseModel<Recipe>>(query)
            .bind(&name)
            .bind(&summary)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<BaseModel<Recipe>> {
        let query = "
            UPDATE recipes
            SET
                name = COALESCE($1, name),
                summary = COALESCE($2, summary),
            WHERE id = $3
            RETURNING *;
        ";
        let recipe = sqlx::query_as::<_, BaseModel<Recipe>>(query)
            .bind(&name)
            .bind(&summary)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe)
    }
}
