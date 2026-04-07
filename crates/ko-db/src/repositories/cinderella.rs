//! Repository for Cinderella War (Fun Class) event data.
//!
//! Provides load-all methods for settings, items, rewards, and stat presets.

use sqlx::PgPool;

use crate::models::cinderella::{
    CindwarItemRow, CindwarRewardItemRow, CindwarRewardRow, CindwarSettingRow, CindwarStatRow,
};

/// Repository for Cinderella War event tables.
pub struct CinderellaRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CinderellaRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all tier settings (5 rows).
    pub async fn load_all_settings(&self) -> Result<Vec<CindwarSettingRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarSettingRow>(
            "SELECT setting_id, playtime, preparetime, min_level, max_level, \
             req_money, req_loyalty, max_user_limit, zone_id, beginner_level \
             FROM cindwar_setting ORDER BY setting_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all equipment items for a given tier.
    pub async fn load_items_by_tier(&self, tier: i16) -> Result<Vec<CindwarItemRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarItemRow>(
            "SELECT tier, id, class, slot_id, item_id, item_count, \
             item_duration, item_flag, item_expire \
             FROM cindwar_items WHERE tier = $1 ORDER BY id",
        )
        .bind(tier)
        .fetch_all(self.pool)
        .await
    }

    /// Load all equipment items across all tiers.
    pub async fn load_all_items(&self) -> Result<Vec<CindwarItemRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarItemRow>(
            "SELECT tier, id, class, slot_id, item_id, item_count, \
             item_duration, item_flag, item_expire \
             FROM cindwar_items ORDER BY tier, id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all rank-based rewards (20 rows).
    pub async fn load_all_rewards(&self) -> Result<Vec<CindwarRewardRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarRewardRow>(
            "SELECT rank_id, exp_count, cash_count, loyalty_count, money_count \
             FROM cindwar_reward ORDER BY rank_id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all reward items.
    pub async fn load_all_reward_items(&self) -> Result<Vec<CindwarRewardItemRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarRewardItemRow>(
            "SELECT rank_id, slot, item_id, item_count, item_duration, item_expiration \
             FROM cindwar_reward_item ORDER BY rank_id, slot",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all stat/skill presets (16 rows).
    pub async fn load_all_stats(&self) -> Result<Vec<CindwarStatRow>, sqlx::Error> {
        sqlx::query_as::<_, CindwarStatRow>(
            "SELECT id, setting_id, class, skill_freepoint, \
             skill_page1, skill_page2, skill_page3, skill_page4, \
             stat_str, stat_sta, stat_dex, stat_int, stat_cha, stat_freepoint \
             FROM cindwar_stat ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }
}
