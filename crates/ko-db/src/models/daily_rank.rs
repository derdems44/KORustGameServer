//! Daily rank models — maps to `daily_rank` and `draki_tower_daily_rank` tables.
//!
//! C++ Reference: `_DAILY_RANK` struct in `GameDefine.h:4468-4487`.

/// A cached daily rank entry from the `daily_rank` table.
///
/// Each row stores the current and previous rank position per rank type
/// for a single character. Rank position is 1-based (1 = best).
/// C++ equivalent: `_DAILY_RANK` (GameDefine.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DailyRankRow {
    /// Character name (primary key).
    pub char_id: String,
    /// Grand Merchant current rank position.
    pub gm_rank_cur: i32,
    /// Grand Merchant previous rank position.
    pub gm_rank_prev: i32,
    /// Monster Hunter current rank position.
    pub mh_rank_cur: i32,
    /// Monster Hunter previous rank position.
    pub mh_rank_prev: i32,
    /// Shozin current rank position.
    pub sh_rank_cur: i32,
    /// Shozin previous rank position.
    pub sh_rank_prev: i32,
    /// Knight Adonis current rank position.
    pub ak_rank_cur: i32,
    /// Knight Adonis previous rank position.
    pub ak_rank_prev: i32,
    /// Hero of Chaos (CW) current rank position.
    pub cw_rank_cur: i32,
    /// Hero of Chaos (CW) previous rank position.
    pub cw_rank_prev: i32,
    /// Disciple of Keron (Upgrade) current rank position.
    pub up_rank_cur: i32,
    /// Disciple of Keron (Upgrade) previous rank position.
    pub up_rank_prev: i32,
}

/// Per-player raw daily rank stats from the `user_daily_rank_stats` table.
///
/// C++ equivalent: `_USER_DAILY_RANK` (GameDefine.h:4457-4466).
/// These cumulative stats are loaded on login and saved on logout.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDailyRankStatsRow {
    /// Character name (primary key).
    pub char_id: String,
    /// Total gold earned from merchant sales.
    pub gm_total_sold: i64,
    /// Total monster kills.
    pub mh_total_kill: i64,
    /// Total crafting/exchange successes.
    pub sh_total_exchange: i64,
    /// Total chaos war first-place wins.
    pub cw_counter_win: i64,
    /// Total blessing event counter.
    pub up_counter_bles: i64,
}

/// A Draki Tower daily rank entry from the `draki_tower_daily_rank` table.
///
/// Rankings are filtered by class and sorted by stage DESC, time ASC.
/// C++ equivalent: `_DRAKI_TOWER_FORDAILY_RANKING` (GameDefine.h:4450-4455).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DrakiTowerDailyRankRow {
    /// Character name (primary key).
    pub char_id: String,
    /// Character class ID (for class-filtered ranking).
    pub class_id: i32,
    /// Highest Draki Tower stage reached.
    pub draki_stage: i16,
    /// Time taken (in seconds or ticks) to reach the stage.
    pub draki_time: i32,
}
