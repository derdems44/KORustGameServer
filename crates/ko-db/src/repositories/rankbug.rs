//! RANKBUG repository — loads ranking configuration from PostgreSQL.
//!
//! C++ Reference: `ClickRankBugSet.h` — loads from MSSQL `RANKBUG` table.

use crate::models::rankbug::RankBugConfig;
use crate::DbPool;

/// Repository for `rankbug` configuration table.
pub struct RankBugRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> RankBugRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load the rank bug configuration (first row).
    ///
    /// C++ Reference: `ClickRankBugSet.h:10-21`
    pub async fn load(&self) -> Result<RankBugConfig, sqlx::Error> {
        sqlx::query_as::<_, RankBugConfig>(
            "SELECT border_join, chaos_join, juraid_join, cz_rank, \
             cr_min_comp, cr_max_comp, lottery_join \
             FROM rankbug LIMIT 1",
        )
        .fetch_optional(self.pool)
        .await
        .map(|opt| {
            opt.unwrap_or_else(|| {
                tracing::warn!("rankbug table empty, using defaults");
                RankBugConfig::default()
            })
        })
    }
}
