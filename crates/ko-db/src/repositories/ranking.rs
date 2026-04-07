//! Repository for user personal/knights ranking operations.

use crate::models::ranking::{KnightsRatingRow, UserRankRow};
use crate::DbPool;

/// Repository for ranking table operations.
pub struct RankingRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> RankingRepository<'a> {
    /// Create a new ranking repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Call the `update_ranks()` PostgreSQL function to recalculate rankings.
    ///
    pub async fn update_ranks(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT update_ranks()")
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Compute per-nation clan rankings and write to `knights_rating` table.
    ///
    ///
    /// This replaces the MSSQL stored procedure that populates the ranking table.
    /// Ranks all clans with points > 0, partitioned by nation, ordered by points DESC.
    /// Also updates the `knights.ranking` column in the DB.
    pub async fn compute_knights_rating(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT compute_knights_rating()")
            .execute(self.pool)
            .await?;
        Ok(())
    }

    /// Load all rows from `knights_rating`.
    ///
    /// Returns per-nation ranking rows sorted by nation then rank_pos.
    pub async fn load_knights_rating(&self) -> Result<Vec<KnightsRatingRow>, sqlx::Error> {
        sqlx::query_as::<_, KnightsRatingRow>(
            "SELECT nation, rank_pos, clan_id, points
             FROM knights_rating ORDER BY nation, rank_pos",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all rows from `user_personal_rank`.
    ///
    /// Returns rank_pos → (karus_user_id, elmo_user_id) mapping.
    pub async fn load_user_personal_rank(&self) -> Result<Vec<UserRankRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRankRow>(
            "SELECT rank_pos, rank_name, elmo_user_id, karus_user_id
             FROM user_personal_rank ORDER BY rank_pos",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all rows from `user_knights_rank`.
    ///
    /// Returns rank_pos → (karus_user_id, elmo_user_id) mapping.
    pub async fn load_user_knights_rank(&self) -> Result<Vec<UserRankRow>, sqlx::Error> {
        sqlx::query_as::<_, UserRankRow>(
            "SELECT rank_pos, rank_name, elmo_user_id, karus_user_id
             FROM user_knights_rank ORDER BY rank_pos",
        )
        .fetch_all(self.pool)
        .await
    }
}
