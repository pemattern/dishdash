pub mod auth;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod state;

pub async fn bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().compact().init();
    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPoolOptions::new().connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let state = crate::state::AppState::new(&pool);
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("server successfully started");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
