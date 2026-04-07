//! Timed notice repository — loads periodic server announcements from PostgreSQL.

use crate::models::timed_notice::TimedNoticeRow;
use crate::DbPool;

/// Repository for timed notice table access.
pub struct TimedNoticeRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> TimedNoticeRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all timed notices (bulk load at startup).
    ///
    /// Returns all rows ordered by index. Empty result means no notices configured.
    pub async fn load_all(&self) -> Result<Vec<TimedNoticeRow>, sqlx::Error> {
        sqlx::query_as::<_, TimedNoticeRow>(
            "SELECT n_index, notice_type, zone_id, notice, time_minutes \
             FROM timed_notice ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
