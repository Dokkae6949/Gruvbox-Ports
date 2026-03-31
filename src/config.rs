use serde::Deserialize;

use crate::types::{Email, RawPassword};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
}

impl Scheme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scheme::Http => "http",
            Scheme::Https => "https",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub app_scheme: Scheme,
    pub database_url: String,
    #[serde(default = "default_run_migrations")]
    pub database_run_migrations: bool,
    // pub smtp_host: String,
    // pub smtp_port: u16,
    // pub smtp_username: Email,
    // pub smtp_password: RawPassword,
    // pub smtp_from: Email,
}

fn default_run_migrations() -> bool {
    false
}

impl Config {
    /// Attempts to load the application configuration from the
    /// system environment and/or the `.env` file. Panics if any
    /// error occurs during initialization.
    pub fn from_env() -> Self {
        dotenvy::dotenv().expect(".env file must be present");
        envy::from_env().expect("required environment variables must be provided")
    }

    pub fn app_addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    pub fn app_url(&self) -> String {
        format!(
            "{}://{}:{}",
            self.app_scheme.as_str(),
            self.app_host,
            self.app_port
        )
    }
}
