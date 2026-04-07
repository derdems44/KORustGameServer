//! Game options repository — server configuration singleton.
//!
//! Source: MSSQL `GAME_OPTIONS` (1 row)

use crate::models::game_options::GameOptions;
use crate::DbPool;

/// Repository for `game_options` table access.
pub struct GameOptionsRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> GameOptionsRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load the singleton game options row.
    pub async fn load(&self) -> Result<GameOptions, sqlx::Error> {
        sqlx::query_as::<_, GameOptions>(
            "SELECT id, maintenance_mode, char_select_login, open_otp, auto_register, \
             free_limit, total_user_limit, server_ip FROM game_options WHERE id = 1",
        )
        .fetch_one(self.pool)
        .await
    }

    /// Update maintenance mode flag.
    pub async fn set_maintenance_mode(&self, enabled: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE game_options SET maintenance_mode = $1 WHERE id = 1")
            .bind(enabled)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Update user limits.
    pub async fn set_user_limits(
        &self,
        free_limit: i16,
        total_limit: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE game_options SET free_limit = $1, total_user_limit = $2 WHERE id = 1")
            .bind(free_limit)
            .bind(total_limit)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
