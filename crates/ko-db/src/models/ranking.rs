/// A row from `user_personal_rank` or `user_knights_rank`.
/// Both tables share the same column layout (dual-nation per row).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRankRow {
    pub rank_pos: i16,
    pub rank_name: String,
    pub elmo_user_id: String,
    pub karus_user_id: String,
}

/// A row from `knights_rating` — per-nation clan ranking.
/// C++ fields: `nRank` (u32), `sClanID` (u16), `nPoints` (u32).
/// Our table adds `nation` for per-nation ranking support.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsRatingRow {
    /// Nation (1=Karus, 2=El Morad).
    pub nation: i16,
    /// Rank position within nation (1 = top).
    pub rank_pos: i32,
    /// Clan ID.
    pub clan_id: i16,
    /// Clan points at time of ranking.
    pub points: i32,
}
