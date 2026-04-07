//! Extended item upgrade repository — loads upgrade probability config from PostgreSQL.
//! - `GameServer/LoadServerData.cpp` — `LoadItemUpProbability()`

use crate::models::item_upgrade_ext::ItemUpProbabilityRow;
use crate::DbPool;

/// Repository for extended item upgrade table access.
pub struct ItemUpgradeExtRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ItemUpgradeExtRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all item upgrade probability rows (bulk load at startup).
    ///
    /// Returns all itemup_probability entries (typically 1 row).
    pub async fn load_all_itemup_probability(
        &self,
    ) -> Result<Vec<ItemUpProbabilityRow>, sqlx::Error> {
        sqlx::query_as::<_, ItemUpProbabilityRow>(
            "SELECT b_type, max_success, max_fail, cur_success, cur_fail \
             FROM itemup_probability ORDER BY b_type",
        )
        .fetch_all(self.pool)
        .await
    }
}
