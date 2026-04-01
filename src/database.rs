use crate::config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Create database connection pool and run optional migrations
/// based on the provided application configuration value `config.database_migrations`.
pub async fn initialize(config: &Config) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await?;

    if config.database_migrations {
        tracing::debug!("Running database migrations");
        sqlx::migrate!("./migrations").run(&pool).await?;
        tracing::info!("Migrations completed successfully");
    }

    Ok(pool)
}
