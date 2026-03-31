use askama::Template;

#[derive(Debug, Clone)]
pub struct CategoryView {
    pub name: String,
    pub label: String,
    pub active: bool,
}

impl CategoryView {
    pub fn new(name: impl Into<String>, label: impl Into<String>, active: bool) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
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

impl PortView {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        author: impl Into<String>,
        url: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            author: author.into(),
            url: url.into(),
            category: category.into(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexPage {
    pub categories: Vec<CategoryView>,
    pub ports: Vec<PortView>,
    pub search: Option<String>,
}

#[derive(Template)]
#[template(path = "partials/main-content.html")]
pub struct MainContentPartial {
    pub categories: Vec<CategoryView>,
    pub ports: Vec<PortView>,
    pub search: Option<String>,
}
