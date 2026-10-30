use sqlx::PgPool;

use crate::services::{
    auth::AuthService, ingredient::IngredientService, recipe::RecipeService, user::UserService,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub ingredient_service: IngredientService,
    pub recipe_service: RecipeService,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(pool: &PgPool) -> Self {
        let auth_service = AuthService::new(pool);
        let ingredient_service = IngredientService::new(pool);
        let recipe_service = RecipeService::new(pool);
        let user_service = UserService::new(pool);
        Self {
            auth_service,
            user_service,
            recipe_service,
            ingredient_service,
        }
    }
}
