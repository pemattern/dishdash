use crate::{
    error::RepositoryResult,
    models::{BaseModel, recipe_ingredient::RecipeIngredient},
    repositories::{Insert, PostgresRepository, Repository, Update},
    schemas::recipe_ingredient::PatchRecipeIngredientRequest,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RecipeIngredientRepository:
    Repository<RecipeIngredient> + Insert<RecipeIngredient> + Update<RecipeIngredient>
{
}

#[async_trait]
impl RecipeIngredientRepository for PostgresRepository<RecipeIngredient> {}

#[async_trait]
impl Insert<RecipeIngredient> for PostgresRepository<RecipeIngredient> {
    async fn insert(
        &self,
        recipe_ingredient: RecipeIngredient,
    ) -> RepositoryResult<BaseModel<RecipeIngredient>> {
        let query = "
            INSERT INTO recipe_ingredients
                (recipe_id, ingredient_id, unit_id, quantity)
            VALUES
                ($1, $2, $3, $4)
            RETURNING *;";
        let recipe_ingredient = sqlx::query_as::<_, BaseModel<RecipeIngredient>>(query)
            .bind(&recipe_ingredient.recipe_id)
            .bind(&recipe_ingredient.ingredient_id)
            .bind(&recipe_ingredient.unit_id)
            .bind(&recipe_ingredient.quantity)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe_ingredient)
    }
}

#[async_trait]
impl Update<RecipeIngredient> for PostgresRepository<RecipeIngredient> {
    async fn update(
        &self,
        id: Uuid,
        recipe_ingredient: PatchRecipeIngredientRequest,
    ) -> RepositoryResult<BaseModel<RecipeIngredient>> {
        let query = "
            UPDATE recipe_ingredients
            SET
                recipe_id = COALESCE($1, recipe_id),
                ingredient_id = COALESCE($2, ingredient_id),
                unit_id = COALESCE($3, unit_id),
                quantity = COALESCE($4, quantity)
            WHERE id = $5
            RETURNING *;";
        let recipe_ingredient = sqlx::query_as::<_, BaseModel<RecipeIngredient>>(query)
            .bind(&recipe_ingredient.recipe_id)
            .bind(&recipe_ingredient.ingredient_id)
            .bind(&recipe_ingredient.unit_id)
            .bind(&recipe_ingredient.quantity)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(recipe_ingredient)
    }
}
