use crate::{error::RepositoryResult, models::Ingredient};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait IngredientRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Ingredient>>;
    async fn get_all(&self) -> RepositoryResult<Vec<Ingredient>>;
    async fn create(&self, name: String, summary: String) -> RepositoryResult<Ingredient>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<Ingredient>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<Ingredient>;
}

#[derive(Clone)]
pub struct PostgresIngredientRepository {
    pool: PgPool,
}

impl PostgresIngredientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IngredientRepository for PostgresIngredientRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Ingredient>> {
        let query = "
            SELECT * FROM ingredients
            WHERE id = $1;
        ";
        let ingredient = sqlx::query_as::<_, Ingredient>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(ingredient)
    }

    async fn get_all(&self) -> RepositoryResult<Vec<Ingredient>> {
        let query = "
            SELECT * FROM ingredients;
        ";
        let ingredients = sqlx::query_as::<_, Ingredient>(query)
            .fetch_all(&self.pool)
            .await?;
        Ok(ingredients)
    }

    async fn create(&self, name: String, summary: String) -> RepositoryResult<Ingredient> {
        let query = "
            INSERT INTO ingredients (name, summary)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let ingredient = sqlx::query_as::<_, Ingredient>(query)
            .bind(&name)
            .bind(&summary)
            .fetch_one(&self.pool)
            .await?;
        Ok(ingredient)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<Ingredient> {
        let query = "
            UPDATE ingredients
            SET
                name = COALESCE($1, name),
                summary = COALESCE($2, summary),
            WHERE id = $3
            RETURNING *;
        ";
        let ingredient = sqlx::query_as::<_, Ingredient>(query)
            .bind(&name)
            .bind(&summary)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(ingredient)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<Ingredient> {
        let query = "
            DELETE FROM ingredients
            WHERE id = $1
            RETURNING *;
        ";
        let ingredient = sqlx::query_as::<_, Ingredient>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(ingredient)
    }
}
