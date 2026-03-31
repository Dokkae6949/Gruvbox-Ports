#![allow(dead_code, unused_imports)]

use std::any::Any;

use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, filter::Directive, layer::SubscriberExt};

use crate::config::Config;

mod config;
mod db;
mod models;
mod router;
mod templates;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Also loads .env file into environment context which
    // is used by tracing subscriber to initialize its filter.
    let config = Config::from_env();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .compact()
        .init();

    info!("Starting server on {}", config.app_url());

    // Run migrations if enabled
    if config.database_run_migrations {
        info!("Running database migrations...");
        db::run_migrations(&config.database_url).await?;
        info!("Migrations completed successfully");
    }

    let router = Router::new()
        .merge(router::router(config.database_url.clone()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(config.app_addr()).await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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

    info!("Shutdown signal received, starting graceful shutdown");
}
