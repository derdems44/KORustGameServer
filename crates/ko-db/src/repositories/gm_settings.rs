//! Game Master settings repository — per-GM permission flags.
//!
//! Source: MSSQL `GAME_MASTER_SETTINGS` (3 rows)

use crate::models::game_master_settings::GameMasterSettings;
use crate::DbPool;

/// Repository for `game_master_settings` table access.
pub struct GmSettingsRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> GmSettingsRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load GM settings for a specific character.
    pub async fn get_by_char(
        &self,
        char_id: &str,
    ) -> Result<Option<GameMasterSettings>, sqlx::Error> {
        sqlx::query_as::<_, GameMasterSettings>(
            "SELECT * FROM game_master_settings WHERE char_id = $1",
        )
        .bind(char_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Load all GM settings (bulk load at startup).
    pub async fn load_all(&self) -> Result<Vec<GameMasterSettings>, sqlx::Error> {
        sqlx::query_as::<_, GameMasterSettings>(
            "SELECT * FROM game_master_settings ORDER BY char_id",
        )
        .fetch_all(self.pool)
        .await
    }
}
