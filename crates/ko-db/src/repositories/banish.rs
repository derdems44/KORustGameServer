//! Banish of Winner repository — event monster spawn positions.
//!
//! Source: MSSQL `BANISH_OF_WINNER` (10 rows)

use crate::models::banish_of_winner::BanishOfWinner;
use crate::DbPool;

/// Repository for `banish_of_winner` table access.
pub struct BanishRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> BanishRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all banishment spawn entries (bulk load at startup).
    pub async fn load_all(&self) -> Result<Vec<BanishOfWinner>, sqlx::Error> {
        sqlx::query_as::<_, BanishOfWinner>(
            "SELECT idx, sid, nation_id, zone_id, pos_x, pos_z, spawn_count, radius, dead_time \
             FROM banish_of_winner ORDER BY idx",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load banishment spawns for a specific nation.
    pub async fn load_by_nation(&self, nation_id: i16) -> Result<Vec<BanishOfWinner>, sqlx::Error> {
        sqlx::query_as::<_, BanishOfWinner>(
            "SELECT idx, sid, nation_id, zone_id, pos_x, pos_z, spawn_count, radius, dead_time \
             FROM banish_of_winner WHERE nation_id = $1 ORDER BY idx",
        )
        .bind(nation_id)
        .fetch_all(self.pool)
        .await
    }
}
