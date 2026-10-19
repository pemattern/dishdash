use std::env;

use sqlx::postgres::PgPoolOptions;

#[cfg(test)]
use crate::repositories::mocks::MockRepository;
use crate::{
    repositories::{PostgresRepository, Repository},
    services::{auth::AuthService, user::UserService},
};

#[derive(Clone)]
pub struct AppState<R>
where
    R: Repository,
{
    pub auth_service: AuthService<R>,
    pub user_service: UserService<R>,
}

impl AppState<PostgresRepository> {
    pub async fn postgres() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = db_url_from_envs();
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let repository = PostgresRepository::new(pool);
        let auth_service = AuthService::new(repository.clone());
        let user_service = UserService::new(repository.clone());
        Ok(Self {
            auth_service,
            user_service,
        })
    }
}

fn db_url_from_envs() -> String {
    let username = env::var("POSTGRES_USER").unwrap_or("postgres".into());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".into());
    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".into());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".into());
    let db = env::var("POSTGRES_DB").unwrap_or("postgres".into());
    format!("postgres://{username}:{password}@{host}:{port}/{db}")
}

#[cfg(test)]
impl AppState<MockRepository> {
    pub async fn mock() -> Result<Self, Box<dyn std::error::Error>> {
        let repository = crate::repositories::mocks::MockRepository::new();
        let auth_service = AuthService::new(repository.clone());
        let user_service = UserService::new(repository.clone());
        Ok(Self {
            auth_service,
            user_service,
        })
    }
}
