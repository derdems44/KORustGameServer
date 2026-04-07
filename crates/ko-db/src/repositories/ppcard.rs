//! PPCard repository — ppcard_list table access.
//! Handles lookup and atomic redemption of prepaid card codes.

use crate::models::ppcard::PPCardRow;
use sqlx::PgPool;

/// Repository for PPCard (prepaid card / serial code) database operations.
pub struct PPCardRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PPCardRepository<'a> {
    /// Create a new PPCard repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Attempt to redeem a card key. Returns the card row if the key is valid
    /// and unused, and atomically marks it as used. Returns `None` if the key
    /// does not exist or has already been redeemed.
    ///
    /// checks status, marks as used, stores account/character info.
    pub async fn redeem(
        &self,
        card_key: &str,
        account_id: &str,
        char_name: &str,
    ) -> Result<Option<PPCardRow>, sqlx::Error> {
        // Atomic UPDATE ... RETURNING: only succeeds if status = 0 (unused).
        // This prevents race conditions if the same key is submitted twice.
        let row = sqlx::query_as::<_, PPCardRow>(
            "UPDATE ppcard_list \
             SET status = 1, \
                 used_by_account = $2, \
                 used_by_character = $3, \
                 used_at = NOW() \
             WHERE card_key = $1 AND status = 0 \
             RETURNING *",
        )
        .bind(card_key)
        .bind(account_id)
        .bind(char_name)
        .fetch_optional(self.pool)
        .await?;

        Ok(row)
    }
}
