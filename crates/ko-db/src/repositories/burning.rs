//! Burning features repository — event rate multiplier presets.
//!
//! Source: MSSQL `BURNING_FEATURES` (3 rows)

use crate::models::burning_features::BurningFeatures;
use crate::DbPool;

/// Repository for `burning_features` table access.
pub struct BurningRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> BurningRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all burning feature tiers (bulk load at startup).
    pub async fn load_all(&self) -> Result<Vec<BurningFeatures>, sqlx::Error> {
        sqlx::query_as::<_, BurningFeatures>(
            "SELECT burn_level, np_rate, money_rate, exp_rate, drop_rate \
             FROM burning_features ORDER BY burn_level",
        )
        .fetch_all(self.pool)
        .await
    }
}
