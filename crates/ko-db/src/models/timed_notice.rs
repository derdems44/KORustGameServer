//! Timed notice model — maps to the `timed_notice` PostgreSQL table.
//!
//! C++ Reference: `TIMED_NOTICE` MSSQL table — periodic server announcements
//! broadcast to all players or a specific zone at configurable intervals.

/// A timed notice row from the database.
///
/// Each row defines a periodic server announcement with a chat type,
/// message text, target zone (0 = all zones), and interval in minutes.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TimedNoticeRow {
    /// Unique notice index (PK).
    pub n_index: i32,
    /// Chat type for the announcement (e.g., 7 = PUBLIC_CHAT).
    ///
    /// C++ field: `noticetype`
    pub notice_type: i16,
    /// Notice message text.
    ///
    /// C++ field: `notice`
    pub notice: String,
    /// Target zone ID. 0 means broadcast to all zones.
    ///
    /// C++ field: `zoneid`
    pub zone_id: i16,
    /// Broadcast interval in minutes (minimum 1).
    ///
    /// C++ field: `time`
    pub time_minutes: i32,
}
