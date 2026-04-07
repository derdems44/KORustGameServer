//! Anti-AFK NPC list repository.

use sqlx::PgPool;

use crate::models::anti_afk_list::AntiAfkEntry;

/// Repository for anti-AFK NPC list database operations.
pub struct AntiAfkRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AntiAfkRepository<'a> {
    /// Create a new anti-AFK repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all anti-AFK NPC entries (static data, loaded once at startup).
    pub async fn load_all(&self) -> Result<Vec<AntiAfkEntry>, sqlx::Error> {
        sqlx::query_as::<_, AntiAfkEntry>("SELECT idx, npc_id FROM anti_afk_list ORDER BY idx")
            .fetch_all(self.pool)
            .await
    }
}
