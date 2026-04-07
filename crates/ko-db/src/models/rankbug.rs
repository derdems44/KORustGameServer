//! RANKBUG configuration model — ranking system multipliers.

use sqlx::FromRow;

/// Ranking bug/multiplier configuration loaded from the `rankbug` table.
#[derive(Debug, Clone, Default, FromRow)]
pub struct RankBugConfig {
    /// BDW join count multiplier.
    pub border_join: i32,
    /// Chaos Dungeon join count multiplier.
    pub chaos_join: i32,
    /// Juraid Mountain join count multiplier.
    pub juraid_join: i32,
    /// PK zone rank multiplier for out-of-top-10 players.
    ///
    pub cz_rank: i32,
    /// Collection Race minimum competition count.
    pub cr_min_comp: i32,
    /// Collection Race maximum competition count.
    pub cr_max_comp: i32,
    /// Lottery event join multiplier.
    pub lottery_join: i32,
}
