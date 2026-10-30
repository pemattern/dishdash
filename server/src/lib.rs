use std::{error::Error, net::SocketAddr};

use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use crate::state::AppState;

pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod responses;
pub mod routes;
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
    Ok(pool)
}

fn create_router(pool: PgPool) -> Router {
    let state = AppState::new(&pool);
    routes::app(state)
}

#[cfg(test)]
mod tests {}
