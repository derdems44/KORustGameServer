//! Daily reward repository — daily login reward items and user progress.
//! Binary Reference: `HandleDailyRewardGive`, `HandleDailyCumRewardGive`

use crate::models::daily_reward::{DailyReward, DailyRewardCumulative, DailyRewardUserRow};
use crate::DbPool;

/// Repository for daily reward table access.
pub struct DailyRewardRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> DailyRewardRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all daily reward items (25 days, ordered).
    pub async fn load_all(&self) -> Result<Vec<DailyReward>, sqlx::Error> {
        sqlx::query_as::<_, DailyReward>(
            "SELECT day_index, item_id, item_count FROM daily_reward ORDER BY day_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load cumulative reward configuration.
    pub async fn load_cumulative(&self) -> Result<Option<DailyRewardCumulative>, sqlx::Error> {
        sqlx::query_as::<_, DailyRewardCumulative>(
            "SELECT id, item1, item2, item3 FROM daily_reward_cumulative WHERE id = 1",
        )
        .fetch_optional(self.pool)
        .await
    }

    /// Load user daily reward progress (up to 25 rows).
    ///
    pub async fn load_user_progress(
        &self,
        user_id: &str,
    ) -> Result<Vec<DailyRewardUserRow>, sqlx::Error> {
        let rows = sqlx::query_as::<_, DailyRewardUserRow>(
            "SELECT user_id, day_index, claimed, day_of_month, last_claim_month \
             FROM daily_reward_user WHERE user_id = $1 ORDER BY day_index",
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;

        if rows.is_empty() {
            // Insert default 25 rows (all unclaimed) in a single batch
            let mut builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
                "INSERT INTO daily_reward_user (user_id, day_index, claimed, day_of_month, last_claim_month) ",
            );
            builder.push_values(0i16..25, |mut b, i| {
                b.push_bind(user_id)
                    .push_bind(i)
                    .push_bind(false)
                    .push_bind(0i16)
                    .push_bind(0i16);
            });
            builder.push(" ON CONFLICT DO NOTHING");
            builder.build().execute(self.pool).await?;
            // Re-fetch
            return sqlx::query_as::<_, DailyRewardUserRow>(
                "SELECT user_id, day_index, claimed, day_of_month, last_claim_month \
                 FROM daily_reward_user WHERE user_id = $1 ORDER BY day_index",
            )
            .bind(user_id)
            .fetch_all(self.pool)
            .await;
        }

        Ok(rows)
    }

    /// Update a single day's claim status with month tracking.
    ///
    pub async fn update_user_day(
        &self,
        user_id: &str,
        day_index: i16,
        claimed: bool,
        day_of_month: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE daily_reward_user SET claimed = $1, day_of_month = $2, last_claim_month = $3 \
             WHERE user_id = $4 AND day_index = $5",
        )
        .bind(claimed)
        .bind(day_of_month)
        .bind(day_of_month) // last_claim_month shares same update
        .bind(user_id)
        .bind(day_index)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update a single day's claim status with explicit month.
    pub async fn update_user_day_with_month(
        &self,
        user_id: &str,
        day_index: i16,
        claimed: bool,
        day_of_month: i16,
        month: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE daily_reward_user SET claimed = $1, day_of_month = $2, last_claim_month = $3 \
             WHERE user_id = $4 AND day_index = $5",
        )
        .bind(claimed)
        .bind(day_of_month)
        .bind(month)
        .bind(user_id)
        .bind(day_index)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Reset all user progress (monthly reset).
    ///
    /// Clears all 25 days to unclaimed for a fresh cycle.
    pub async fn reset_user_progress(&self, user_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE daily_reward_user SET claimed = false, day_of_month = 0, last_claim_month = 0 \
             WHERE user_id = $1",
        )
        .bind(user_id)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
