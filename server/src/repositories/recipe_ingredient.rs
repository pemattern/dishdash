use crate::{
    error::RepositoryResult,
    models::{BaseModel, recipe_ingredient::RecipeIngredient},
    repositories::PostgresRepository,
    schemas::recipe_ingredient::PatchRecipeIngredientRequest,
};
use uuid::Uuid;

impl PostgresRepository<RecipeIngredient> {
    pub async fn insert(
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

    pub async fn update(
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
