//! Lottery event repository.
//!
//! Source: MSSQL `LOTTERY_EVENT_SETTINGS` (11 rows)

use crate::models::lottery_event::LotteryEventSettings;
use crate::DbPool;

/// Repository for `lottery_event_settings` table access.
pub struct LotteryEventRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> LotteryEventRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all lottery event settings.
    pub async fn load_all(&self) -> Result<Vec<LotteryEventSettings>, sqlx::Error> {
        sqlx::query_as::<_, LotteryEventSettings>(
            "SELECT lnum, req_item1, req_item_count1, req_item2, req_item_count2, \
             req_item3, req_item_count3, req_item4, req_item_count4, \
             req_item5, req_item_count5, reward_item1, reward_item2, \
             reward_item3, reward_item4, user_limit, event_time \
             FROM lottery_event_settings ORDER BY lnum",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Get lottery settings by number.
    pub async fn get_by_lnum(
        &self,
        lnum: i16,
    ) -> Result<Option<LotteryEventSettings>, sqlx::Error> {
        sqlx::query_as::<_, LotteryEventSettings>(
            "SELECT lnum, req_item1, req_item_count1, req_item2, req_item_count2, \
             req_item3, req_item_count3, req_item4, req_item_count4, \
             req_item5, req_item_count5, reward_item1, reward_item2, \
             reward_item3, reward_item4, user_limit, event_time \
             FROM lottery_event_settings WHERE lnum = $1",
        )
        .bind(lnum)
        .fetch_optional(self.pool)
        .await
    }
}
