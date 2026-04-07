//! Zone rewards repository — PvP kill and online-time rewards.
//!
//! Source: MSSQL `ZONE_KILL_REWARD` + `ZONE_ONLINE_REWARD`

use crate::models::zone_rewards::{ZoneKillReward, ZoneOnlineReward};
use crate::DbPool;

/// Repository for zone reward table access.
pub struct ZoneRewardsRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> ZoneRewardsRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all zone kill reward entries.
    pub async fn load_kill_rewards(&self) -> Result<Vec<ZoneKillReward>, sqlx::Error> {
        sqlx::query_as::<_, ZoneKillReward>(
            "SELECT idx, zone_id, nation, party_required, all_party_reward, kill_count, \
             item_name, item_id, item_duration, item_count, item_flag, item_expiration, \
             drop_rate, give_to_warehouse, status, is_priest, priest_rate \
             FROM zone_kill_reward ORDER BY idx",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all zone online reward entries.
    pub async fn load_online_rewards(&self) -> Result<Vec<ZoneOnlineReward>, sqlx::Error> {
        sqlx::query_as::<_, ZoneOnlineReward>(
            "SELECT zone_id, item_id, item_count, item_time, minute, loyalty, cash, tl, \
             pre_item_id, pre_item_count, pre_item_time, pre_minute, pre_loyalty, pre_cash, pre_tl \
             FROM zone_online_reward ORDER BY zone_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load kill reward for a specific zone.
    pub async fn load_kill_reward_by_zone(
        &self,
        zone_id: i16,
    ) -> Result<Option<ZoneKillReward>, sqlx::Error> {
        sqlx::query_as::<_, ZoneKillReward>(
            "SELECT idx, zone_id, nation, party_required, all_party_reward, kill_count, \
             item_name, item_id, item_duration, item_count, item_flag, item_expiration, \
             drop_rate, give_to_warehouse, status, is_priest, priest_rate \
             FROM zone_kill_reward WHERE zone_id = $1 AND status = 1",
        )
        .bind(zone_id)
        .fetch_optional(self.pool)
        .await
    }
}
