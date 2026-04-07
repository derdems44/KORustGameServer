//! Collection Race event repository.
//!
//! Source: MSSQL `COLLECTION_RACE_EVENT_SETTINGS` + `COLLECTION_RACE_EVENT_REWARD`

use crate::models::collection_race::{CollectionRaceReward, CollectionRaceSettings};
use crate::DbPool;

/// Repository for collection race event table access.
pub struct CollectionRaceRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> CollectionRaceRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all collection race event settings.
    pub async fn load_settings(&self) -> Result<Vec<CollectionRaceSettings>, sqlx::Error> {
        sqlx::query_as::<_, CollectionRaceSettings>(
            "SELECT event_index, event_name, unit1, unit_count1, unit2, unit_count2, \
             unit3, unit_count3, min_level, max_level, event_zone, event_time, \
             user_limit, is_repeat, auto_start, auto_hour, auto_minute \
             FROM collection_race_settings ORDER BY event_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load rewards for a specific event.
    pub async fn load_rewards_by_event(
        &self,
        event_id: i16,
    ) -> Result<Vec<CollectionRaceReward>, sqlx::Error> {
        sqlx::query_as::<_, CollectionRaceReward>(
            "SELECT idx, event_id, description, item_id, item_count, rate, \
             item_time, item_flag, item_session \
             FROM collection_race_reward WHERE event_id = $1 ORDER BY idx",
        )
        .bind(event_id)
        .fetch_all(self.pool)
        .await
    }
}
