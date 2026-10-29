use crate::services::{auth::AuthService, ingredient::IngredientService, user::UserService};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub ingredient_service: IngredientService,
}

impl AppState {
    pub fn new(pool: &PgPool) -> Self {
        let auth_service = AuthService::new(pool);
        let user_service = UserService::new(pool);
        let ingredient_service = IngredientService::new(pool);
        Self {
            auth_service,
            user_service,
            ingredient_service,
        }
    }
}
