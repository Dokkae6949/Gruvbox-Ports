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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePort {
    pub name: String,
    pub description: String,
    pub author: String,
    pub url: String,
    pub category: String,
}

impl Port {
    /// Get all ports with optional category filtering and fuzzy search on name only
    pub async fn find_all(
        conn: &mut PgConnection,
        category: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<Port>, sqlx::Error> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, name, description, author, url, category, created_at, updated_at FROM ports WHERE 1=1",
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
            query.push(") >= 0.15");
            query.push(" ORDER BY similarity(name, ");
            query.push_bind(term);
            query.push(") DESC");
        } else {
            query.push(" ORDER BY name ASC");
        }

        query.build_query_as::<Port>().fetch_all(conn).await
    }

    /// Get a single port by ID
    pub async fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Port, sqlx::Error> {
        sqlx::query_as::<_, Port>(
            "SELECT id, name, description, author, url, category, created_at, updated_at 
             FROM ports WHERE id = $1",
        )
        .bind(id)
        .fetch_one(conn)
        .await
    }

    /// Create a new port
    pub async fn create(conn: &mut PgConnection, data: CreatePort) -> Result<Port, sqlx::Error> {
        sqlx::query_as::<_, Port>(
            "INSERT INTO ports (name, description, author, url, category) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, name, description, author, url, category, created_at, updated_at",
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.author)
        .bind(data.url)
        .bind(data.category)
        .fetch_one(conn)
        .await
    }

    /// Update an existing port
    pub async fn update(
        conn: &mut PgConnection,
        id: Uuid,
        data: CreatePort,
    ) -> Result<Port, sqlx::Error> {
        sqlx::query_as::<_, Port>(
            "UPDATE ports 
             SET name = $1, description = $2, author = $3, url = $4, category = $5, updated_at = NOW() 
             WHERE id = $6 
             RETURNING id, name, description, author, url, category, created_at, updated_at"
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.author)
        .bind(data.url)
        .bind(data.category)
        .bind(id)
        .fetch_one(conn)
        .await
    }

    /// Delete a port
    pub async fn delete(conn: &mut PgConnection, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM ports WHERE id = $1")
            .bind(id)
            .execute(conn)
            .await?;
        Ok(())
    }
}
