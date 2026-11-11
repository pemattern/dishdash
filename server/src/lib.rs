use std::{error::Error, net::SocketAddr};

use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use crate::{
    seed_data::{upsert_ingredient_names_seed_data, upsert_ingredient_seed_data},
    state::AppState,
};

pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod requests;
pub mod responses;
pub mod routes;
pub mod seed_data;
pub mod services;
pub mod state;

const DATABASE_URL_ENV_KEY: &str = "DATABASE_URL";
const IP_PORT: &str = "0.0.0.0:3000";

pub async fn bootstrap() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().compact().init();
    dotenvy::dotenv()?;
    let pool = connect_postgres().await?;
    let router = create_router(pool);
    let listener = TcpListener::bind(IP_PORT).await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

async fn connect_postgres() -> Result<PgPool, Box<dyn Error>> {
    let url = std::env::var(DATABASE_URL_ENV_KEY)?;
    let pool = PgPoolOptions::new().connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;
    upsert_ingredient_seed_data(&pool).await?;
    upsert_ingredient_names_seed_data(&pool).await?;
    log::info!("completed updating seed data");
    Ok(pool)
}

fn create_router(pool: PgPool) -> Router {
    let state = AppState::new(&pool);
    routes::app(state)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use sqlx::PgPool;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        auth::token::generate_access_token, create_router,
        models::recipe::schemas::PostRecipeSchema,
    };

    #[sqlx::test]
    async fn insert_recipe(pool: PgPool) {
        let app = create_router(pool);
        let token = generate_access_token(Uuid::nil()).unwrap();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/recipes")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::from(r#"{"name":"Pumpkin Soup", "summary":"tasty"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let schema: PostRecipeSchema = serde_json::from_slice(&body).unwrap();
        assert_eq!(schema.name, "Pumpkin Soup");
        assert_eq!(schema.summary, "tasty");
    }
}
