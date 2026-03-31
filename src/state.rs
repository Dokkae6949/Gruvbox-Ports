use sqlx::PgPool;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}
