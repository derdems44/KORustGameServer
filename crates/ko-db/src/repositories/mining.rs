//! Mining & Fishing item drop table repository.

use crate::models::{MiningExchangeRow, MiningFishingItemRow};
use crate::DbPool;

/// Repository for mining/fishing item drop data.
pub struct MiningRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> MiningRepository<'a> {
    /// Create a new mining repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all mining/fishing item rows from the database.
    ///
    pub async fn load_all_mining_items(&self) -> Result<Vec<MiningFishingItemRow>, sqlx::Error> {
        sqlx::query_as::<_, MiningFishingItemRow>(
            "SELECT n_index, n_table_type, n_war_status, use_item_type, \
             n_give_item_name, n_give_item_id, n_give_item_count, success_rate \
             FROM mining_fishing_item ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all mining exchange (ore craft) rows from the database.
    ///
    pub async fn load_all_mining_exchanges(&self) -> Result<Vec<MiningExchangeRow>, sqlx::Error> {
        sqlx::query_as::<_, MiningExchangeRow>(
            "SELECT n_index, s_npc_id, give_effect, ore_type, \
             n_origin_item_num, n_give_item_num, n_give_item_count, success_rate \
             FROM mining_exchange ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
