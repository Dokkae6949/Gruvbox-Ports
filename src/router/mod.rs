use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::{Connection, PgConnection};
use tower_http::services::ServeDir;

use crate::{
    models,
    templates::{CategoryView, IndexPage, MainContentPartial, PortView},
};

pub fn router(database_url: String) -> Router {
    Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(database_url)
}

#[derive(Debug, Deserialize)]
struct FilterQuery {
    pub category: Option<String>,
    pub search: Option<String>,
}

async fn index(
    State(database_url): State<String>,
    Query(query): Query<FilterQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let mut conn = PgConnection::connect(&database_url).await.unwrap();

    // Get all categories from database
    let db_categories = models::Category::find_all(&mut conn).await.unwrap_or_default();

    let active_category = query.category.as_deref().unwrap_or("all");

    // Build category views with "all" option
    let mut categories = vec![CategoryView::new("all", "All", active_category == "all")];

    for cat in db_categories {
        categories.push(CategoryView::new(
            &cat.id,
            &cat.label,
            cat.id == active_category,
        ));
    }

    // Query ports from database with filtering and search
    let category_filter = if active_category == "all" {
        None
    } else {
        Some(active_category)
    };

    let db_ports = models::Port::find_all(&mut conn, category_filter, query.search.as_deref())
        .await
        .unwrap_or_default();

    // Convert database models to view models
    let ports: Vec<PortView> = db_ports
        .into_iter()
        .map(|p| PortView::new(&p.name, &p.description, &p.author, &p.url, &p.category))
        .collect();

    let template = if headers.contains_key("hx-request") {
        // HTMX request - return main content partial
        MainContentPartial {
            categories,
            ports,
            search: query.search.clone(),
        }
        .render()
    } else {
        // Regular request - return full page
        IndexPage {
            categories,
            ports,
            search: query.search.clone(),
        }
        .render()
    }
    .unwrap();

    Html(template)
}
