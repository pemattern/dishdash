use crate::services::{auth::AuthService, postgres::PostgresService};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub postgres_service: PostgresService,
}

impl AppState {
    pub fn new(pool: &PgPool) -> Self {
        let auth_service = AuthService::new(pool);
        let postgres_service = PostgresService::new(pool);
        Self {
            auth_service,
            postgres_service,
        }
    }
}
