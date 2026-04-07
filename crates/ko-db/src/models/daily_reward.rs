//! Daily reward models — maps to `daily_reward`, `daily_reward_cumulative`,
//! and `daily_reward_user` PostgreSQL tables.

/// A row from the `daily_reward` table — defines the item reward for
/// each daily login day (0–24).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyReward {
    /// Day index in the daily reward cycle (0-based, 0–24).
    pub day_index: i16,
    /// Item template ID to reward for this day.
    pub item_id: i32,
    /// Number of items to give (default 1).
    pub item_count: i16,
}

/// A row from the `daily_reward_cumulative` table — defines cumulative
/// milestone rewards (3 bonus items).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyRewardCumulative {
    /// Primary key (always 1, singleton row).
    pub id: i32,
    /// First cumulative reward item ID.
    pub item1: Option<i32>,
    /// Second cumulative reward item ID.
    pub item2: Option<i32>,
    /// Third cumulative reward item ID.
    pub item3: Option<i32>,
}

/// A row from the `daily_reward_user` table — per-user daily reward progress.
/// - type: 0 = unclaimed, 1 = claimed
/// - day: day-of-month when claimed (0 = never)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyRewardUserRow {
    /// Character name (user_id).
    pub user_id: String,
    /// Day index (0–24).
    pub day_index: i16,
    /// Whether claimed (C++ sbType).
    pub claimed: bool,
    /// Day-of-month when claimed (C++ sGetDay).
    pub day_of_month: i16,
    /// Month number when last claimed (1–12). Used for monthly reset detection.
    pub last_claim_month: i16,
}
