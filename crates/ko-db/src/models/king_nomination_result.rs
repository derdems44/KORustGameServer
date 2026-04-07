//! King election nomination results.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct KingNominationResult {
    pub id: i32,
    pub nation: i16,
    pub user_id: String,
    pub rank: i16,
    pub clan_id: i16,
    pub month: i16,
    pub year: i16,
    pub king_votes: i32,
    pub total_votes: i32,
}
