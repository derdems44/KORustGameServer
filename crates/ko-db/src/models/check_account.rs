//! Check account model — maps to the `check_account` PostgreSQL table.
//!
//! Source: MSSQL `CHECK_ACCOUNT` table — GM ban/restriction tracking per account.

use chrono::{DateTime, Utc};

/// A row from the `check_account` table — tracks account restrictions,
/// bans, and GM moderation actions.
///
/// Each row stores the ban/open history for a single account, including
/// the GM who last modified it, the reason, and login time restrictions.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CheckAccount {
    /// Account ID string (PK).
    pub account_id: String,
    /// GM character name who issued the action.
    pub gm: String,
    /// Login time restriction status (0=normal, other=restricted).
    pub login_time_status: i32,
    /// Reason text for the ban/restriction.
    pub reason: String,
    /// Number of times this account has been banned.
    pub ban_count: i32,
    /// Number of times this account has been unbanned/opened.
    pub open_count: i32,
    /// Timestamp of the last update to this record.
    pub updated_at: DateTime<Utc>,
}
