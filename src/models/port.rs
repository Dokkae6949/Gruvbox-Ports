use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgConnection};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Port {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub author: String,
    pub url: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Port {
    /// Get all ports with optional category filtering and fuzzy search on name only
    pub async fn find_all(
        conn: &mut PgConnection,
        category: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<Port>, sqlx::Error> {
        let mut query = sqlx::QueryBuilder::new(
            r#"
            SELECT id, name, description, author, url, category, created_at, updated_at
            FROM ports
            WHERE 1=1
            "#,
        );

        // Category filter
        if let Some(cat) = category {
            if cat != "all" {
                query.push(" AND category = ");
                query.push_bind(cat);
            }
        }

        // Fuzzy search on name only using trigram similarity with threshold
        let search_term = search.filter(|s| !s.is_empty());
        if let Some(term) = search_term {
            query.push(" AND similarity(name, ");
            query.push_bind(term);
            query.push(") >= 0.15"); // Threshold hardcoded for now
            query.push(" ORDER BY similarity(name, ");
            query.push_bind(term);
            query.push(") DESC");
        } else {
            query.push(" ORDER BY name ASC");
        }

        query.build_query_as::<Port>().fetch_all(conn).await
    }

    pub async fn count(conn: &mut PgConnection) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT count(*) FROM ports")
            .fetch_one(conn)
            .await
    }
}
