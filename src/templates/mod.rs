use askama::Template;

use crate::models::{Category, Port};

#[derive(Debug, Clone)]
pub struct CategoryView {
    pub name: String,
    pub label: String,
    pub active: bool,
}

impl CategoryView {
    pub fn all(active: bool) -> Self {
        Self {
            name: "all".into(),
            label: "All".into(),
            active,
        }
    }

    pub fn from_model(category: &Category, active: bool) -> Self {
        Self {
            name: category.id.clone(),
            label: category.label.clone(),
            active,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PortView {
    pub name: String,
    pub description: String,
    pub author: String,
    pub url: String,
    pub category: String,
}

impl From<&Port> for PortView {
    fn from(port: &Port) -> Self {
        Self {
            name: port.name.clone(),
            description: port.description.clone(),
            author: port.author.clone(),
            url: port.url.clone(),
            category: port.category.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {
    pub categories: Vec<CategoryView>,
    pub ports: Vec<PortView>,
    pub search: Option<String>,
    pub port_count: i64,
}

#[derive(Template)]
#[template(path = "partials/main-content.html")]
pub struct MainContentPartial {
    pub categories: Vec<CategoryView>,
    pub ports: Vec<PortView>,
    pub search: Option<String>,
}
