use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub label: String,
    pub display_order: i32,
    pub created_at: DateTime<Utc>,
}

impl Category {
    /// Get all categories ordered by display_order
    pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, label, display_order, created_at 
             FROM categories 
             ORDER BY display_order ASC",
        )
        .fetch_all(conn)
        .await
    }
}
