//! BDW event timer repository.
//!
//! Source: MSSQL `EVENT_BEEF_PLAY_TIMER` (1 row)

use crate::models::event_beef_play_timer::EventBeefPlayTimer;
use crate::DbPool;

/// Repository for `event_beef_play_timer` table access.
pub struct EventBeefTimerRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> EventBeefTimerRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load BDW timer configuration.
    pub async fn load(&self) -> Result<Option<EventBeefPlayTimer>, sqlx::Error> {
        sqlx::query_as::<_, EventBeefPlayTimer>(
            "SELECT event_local_id, event_zone_id, event_name, monument_time, \
             loser_sign_time, farming_time FROM event_beef_play_timer LIMIT 1",
        )
        .fetch_optional(self.pool)
        .await
    }
}
