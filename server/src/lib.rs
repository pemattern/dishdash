use std::{error::Error, net::SocketAddr};

use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use crate::{seed_data::upsert_seed_data_tables, state::AppState};

pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod payloads;
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
    tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%H:%M:%S%.3f".to_string(),
        ))
        .with_target(false)
        .with_level(true)
        .compact()
        .init();
    dotenvy::dotenv()?;
    let pool = connect_postgres().await?;
    let router = create_router(pool);
    let listener = TcpListener::bind(IP_PORT).await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn connect_postgres() -> Result<PgPool, Box<dyn Error>> {
    let url = std::env::var(DATABASE_URL_ENV_KEY)?;
    let pool = PgPoolOptions::new().connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;
    upsert_seed_data_tables(&pool).await?;
    Ok(pool)
}

fn create_router(pool: PgPool) -> Router {
    let state = AppState::new(&pool);
    routes::app(state)
}

async fn shutdown_signal() {
    use tokio::signal;
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use serde_json::json;
    use sqlx::PgPool;
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        auth::token::generate_access_token, create_router, seed_data::upsert_seed_data_tables,
    };

    #[sqlx::test]
    async fn insert_recipe(pool: PgPool) {
        let app = create_router(pool);
        let token = generate_access_token(Uuid::nil()).unwrap();
        let name = "pumpkin soup";
        let image_data = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+j5ZsAAAAASUVORK5CYII=";
        let duration = 10;
        let json = json!({
            "name": name,
            "imageData": image_data,
            "durationMinutes" : duration,
        });
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/recipes")
                    .header("content-type", "application/json")
                    .header("authorization", format!("bearer {}", token))
                    .body(Body::from(json.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let schema: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(schema["name"], name);
        assert_eq!(schema["durationMinutes"], duration);
    }

    #[sqlx::test]
    async fn search_ingredients(pool: PgPool) {
        upsert_seed_data_tables(&pool).await.unwrap();
        let app = create_router(pool);
        let token = generate_access_token(Uuid::nil()).unwrap();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/ingredients?q=Möhr&locale=de")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let schema: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(schema[0]["name"], "Karotte");
        assert_eq!(schema[0]["alias"], "Möhre");
    }
}
