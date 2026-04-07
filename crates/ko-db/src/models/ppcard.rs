//! PPCard model — maps to the `ppcard_list` PostgreSQL table.

use chrono::{DateTime, Utc};

/// A single PPCard (prepaid card) entry from the `ppcard_list` table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PPCardRow {
    /// The 20-character card key (4-digit prefix + 16-char suffix).
    pub card_key: String,
    /// Knight Cash amount to award on redemption.
    pub knight_cash: i32,
    /// TL (bonus cash) amount to award on redemption.
    pub tl_balance: i32,
    /// Cash type (1, 2, or 3).
    pub cash_type: i16,
    /// 0 = unused, 1 = used/redeemed.
    pub status: i16,
    /// Account that redeemed this card (NULL if unused).
    pub used_by_account: Option<String>,
    /// Character that redeemed this card (NULL if unused).
    pub used_by_character: Option<String>,
    /// Timestamp of redemption (NULL if unused).
    pub used_at: Option<DateTime<Utc>>,
}
