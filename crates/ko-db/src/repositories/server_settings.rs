//! Repository for server settings, damage settings, and home positions.

use crate::models::{
    DamageSettingsRow, HomeRow, ServerSettingsRow, StartPositionRandomRow, StartPositionRow,
};
use crate::DbPool;

/// Repository for server-wide configuration tables.
pub struct ServerSettingsRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ServerSettingsRepository<'a> {
    /// Create a new repository instance.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load the single server settings row.
    ///
    pub async fn load_server_settings(&self) -> Result<ServerSettingsRow, sqlx::Error> {
        sqlx::query_as::<_, ServerSettingsRow>("SELECT * FROM server_settings LIMIT 1")
            .fetch_one(self.pool)
            .await
    }

    /// Load the single damage settings row.
    ///
    pub async fn load_damage_settings(&self) -> Result<DamageSettingsRow, sqlx::Error> {
        sqlx::query_as::<_, DamageSettingsRow>("SELECT * FROM damage_settings LIMIT 1")
            .fetch_one(self.pool)
            .await
    }

    /// Load all home position rows (one per nation).
    pub async fn load_home_positions(&self) -> Result<Vec<HomeRow>, sqlx::Error> {
        sqlx::query_as::<_, HomeRow>("SELECT * FROM home ORDER BY nation")
            .fetch_all(self.pool)
            .await
    }

    /// Load all start position rows (one per zone).
    ///
    pub async fn load_start_positions(&self) -> Result<Vec<StartPositionRow>, sqlx::Error> {
        sqlx::query_as::<_, StartPositionRow>("SELECT * FROM start_position ORDER BY zone_id")
            .fetch_all(self.pool)
            .await
    }

    /// Load all random spawn points for special zones.
    ///
    pub async fn load_start_positions_random(
        &self,
    ) -> Result<Vec<StartPositionRandomRow>, sqlx::Error> {
        sqlx::query_as::<_, StartPositionRandomRow>(
            "SELECT * FROM start_position_random ORDER BY zone_id, id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load right-top title messages from the `right_top_title` table.
    ///
    ///
    /// Returns (title, message) pairs ordered by ID.
    pub async fn load_right_top_titles(&self) -> Result<Vec<(String, String)>, sqlx::Error> {
        #[derive(sqlx::FromRow)]
        struct Row {
            str_title: String,
            str_message: String,
        }
        let rows = sqlx::query_as::<_, Row>(
            "SELECT str_title, str_message FROM right_top_title ORDER BY id",
        )
        .fetch_all(self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.str_title, r.str_message))
            .collect())
    }
}
