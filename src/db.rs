use sqlx::{postgres::PgPoolOptions, PgConnection, PgPool, Connection};
use std::time::Duration;

/// Create a PostgreSQL connection
pub async fn create_connection(database_url: &str) -> Result<PgConnection, sqlx::Error> {
    PgConnection::connect(database_url).await
}

/// Create a PostgreSQL connection pool (for migrations only)
async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

/// Run database migrations
pub async fn run_migrations(database_url: &str) -> Result<(), sqlx::migrate::MigrateError> {
    let pool = create_pool(database_url).await
        .map_err(|e| sqlx::migrate::MigrateError::Execute(e.into()))?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
}
