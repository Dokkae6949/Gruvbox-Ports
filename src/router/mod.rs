use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

use crate::{
    models::{Category, Port},
    state::AppState,
    templates::{CategoryView, IndexPage, MainContentPartial, PortView},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"))
}

#[derive(Debug, Deserialize)]
struct FilterQuery {
    category: Option<String>,
    search: Option<String>,
}

async fn index(
    State(state): State<AppState>,
    Query(query): Query<FilterQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let mut conn = state.pool.acquire().await.unwrap();

    let db_categories = Category::find_all(&mut conn).await.unwrap_or_default();
    let category_filter = query.category.as_deref();
    let db_ports = Port::find_all(&mut conn, category_filter, query.search.as_deref())
        .await
        .unwrap_or_default();

    let is_all = category_filter.is_none() || category_filter == Some("all");
    let categories: Vec<CategoryView> = std::iter::once(CategoryView::all(is_all))
        .chain(
            db_categories
                .iter()
                .map(|cat| CategoryView::from_model(cat, category_filter == Some(&cat.id))),
        )
        .collect();

    let ports: Vec<PortView> = db_ports.iter().map(PortView::from).collect();

    let is_htmx = headers.contains_key("hx-request");
    let html = if is_htmx {
        MainContentPartial {
            categories,
            ports,
            search: query.search,
        }
        .render()
    } else {
        let port_count = Port::count(&mut conn).await.unwrap_or_default();

        IndexPage {
            categories,
            ports,
            search: query.search,
            port_count,
        }
        .render()
    }
    .unwrap();

    Html(html)
}
