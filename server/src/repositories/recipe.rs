use crate::{error::RepositoryResult, models::Recipe};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Recipe>>;
    async fn create(&self, name: String, summary: String) -> RepositoryResult<Recipe>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        summary: Option<String>,
    ) -> RepositoryResult<Recipe>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<Recipe>;
}

#[derive(Clone)]
pub struct PostgresRecipeRepository {
    pool: PgPool,
}

impl PostgresRecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecipeRepository for PostgresRecipeRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<Recipe>> {
        let query = "
            SELECT * FROM recipes
            WHERE id = $1;
        ";
        let recipe = sqlx::query_as::<_, Recipe>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(recipe)
    }

    async fn create(&self, name: String, summary: String) -> RepositoryResult<Recipe> {
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

    async fn update(
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

    async fn delete(&self, id: Uuid) -> RepositoryResult<Recipe> {
        let query = "
            DELETE FROM recipes
            WHERE id = $1
            RETURNING *;
        ";
        let recipe = sqlx::query_as::<_, Recipe>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe)
    }
}
