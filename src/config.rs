use serde::Deserialize;
use tracing::info;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub app_origin: String,
    pub database_url: String,
    #[serde(default)]
    pub database_migrations: bool,
    // pub smtp_host: String,
    // pub smtp_port: u16,
    // pub smtp_username: Email,
    // pub smtp_password: RawPassword,
    // pub smtp_from: Email,
}

impl Config {
    pub fn from_env() -> Self {
        if dotenvy::dotenv().is_ok() {
            info!("Loaded .env file successfully");
        } else {
            info!("No .env file found");
        }

        envy::from_env().expect("required environment variables must be provided")
    }

    pub fn app_addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    pub fn app_origin(&self) -> &str {
        &self.app_origin
    }
}
