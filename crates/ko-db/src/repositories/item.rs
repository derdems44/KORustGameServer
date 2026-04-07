//! Item repository — ITEM table access (static game data).

use sqlx::PgPool;

use crate::models::Item;

/// Repository for item definition lookups.
pub struct ItemRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ItemRepository<'a> {
    /// Create a new item repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find an item definition by its numeric ID.
    pub async fn find_by_num(&self, num: i32) -> Result<Option<Item>, sqlx::Error> {
        sqlx::query_as::<_, Item>("SELECT * FROM item WHERE num = $1")
            .bind(num)
            .fetch_optional(self.pool)
            .await
    }

    /// Load all items (for server startup cache).
    pub async fn load_all(&self) -> Result<Vec<Item>, sqlx::Error> {
        sqlx::query_as::<_, Item>("SELECT * FROM item ORDER BY num")
            .fetch_all(self.pool)
            .await
    }

    /// Find items by kind (weapon, armor, etc.).
    pub async fn find_by_kind(&self, kind: i32) -> Result<Vec<Item>, sqlx::Error> {
        sqlx::query_as::<_, Item>("SELECT * FROM item WHERE kind = $1 ORDER BY num")
            .bind(kind)
            .fetch_all(self.pool)
            .await
    }
}
