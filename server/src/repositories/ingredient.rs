use crate::{
    error::RepositoryResult,
    models::{BaseModel, ingredient::Ingredient},
    repositories::{PostgresRepository, Repository},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait IngredientRepository: Repository<Ingredient> {
    async fn create(&self, name: String) -> RepositoryResult<BaseModel<Ingredient>>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
    ) -> RepositoryResult<BaseModel<Ingredient>>;
}

#[async_trait]
impl IngredientRepository for PostgresRepository<Ingredient> {
    async fn create(&self, name: String) -> RepositoryResult<BaseModel<Ingredient>> {
        let query = "
            INSERT INTO ingredients (name)
            VALUES ($1)
            RETURNING *;";
        let ingredient = sqlx::query_as::<_, BaseModel<Ingredient>>(query)
            .bind(&name)
            .fetch_one(&self.pool)
            .await?;
        Ok(ingredient)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
    ) -> RepositoryResult<BaseModel<Ingredient>> {
        let query = "
            UPDATE ingredients
            SET
                name = COALESCE($1, name),
            WHERE id = $2
            RETURNING *;";
        let ingredient = sqlx::query_as::<_, BaseModel<Ingredient>>(query)
            .bind(&name)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(ingredient)
    }
}
