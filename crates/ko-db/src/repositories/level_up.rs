//! Level-up repository -- loads the level-up experience table from PostgreSQL.
//!
//! C++ Reference:
//! - `GameServer/LoadServerData.cpp` -- `LoadLevelUpTable()`
//! - `shared/database/LevelUpTableSet.h` -- `CLevelUpTableSet`

use crate::models::LevelUpRow;
use crate::DbPool;

/// Repository for `level_up` table access.
pub struct LevelUpRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> LevelUpRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all level-up rows (bulk load at startup).
    ///
    /// Returns one row per (level, rebirth_level) combination (93 rows).
    /// C++ Reference: `CGameServerDlg::LoadLevelUpTable()`
    pub async fn load_all(&self) -> Result<Vec<LevelUpRow>, sqlx::Error> {
        sqlx::query_as::<_, LevelUpRow>(
            "SELECT id, level, exp, rebirth_level \
             FROM level_up ORDER BY rebirth_level, level",
        )
        .fetch_all(self.pool)
        .await
    }
}
