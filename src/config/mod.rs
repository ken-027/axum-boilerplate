use anyhow::Result;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let server = ServerConfig {
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        };

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://axum_user:axum_password@localhost:5433/axum_db".to_string()
            }),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
        };

        let jwt = JwtConfig {
            secret: env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
            expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()?,
        };

        Ok(Self {
            server,
            database,
            jwt,
        })
    }

    pub async fn setup_database(&self) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(self.database.max_connections)
            .connect(&self.database.url)
            .await?;

        tracing::info!("Database connection established");
        Ok(pool)
    }
}
