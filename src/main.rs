mod config;
mod handlers;
mod middleware;
mod repositories;
mod utils;

use axum::{http::StatusCode, response::Json, routing::get, Router};
use config::Config;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_boilerplate=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let config = Config::from_env()?;
    let db_pool = config.setup_database().await?;

    sqlx::migrate!("./migrations").run(&db_pool).await?;

    let app = create_app(db_pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .merge(handlers::auth::routes())
        .merge(handlers::users::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(db_pool)
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    })))
}
