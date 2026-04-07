//! Costume repository — user costume appearance state persistence.
//!
//! v2525 WIZ_COSTUME (0xC3) — equippable costume with dye colors.

use crate::models::costume::UserCostume;
use crate::DbPool;

/// Repository for user_costume table access.
pub struct CostumeRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> CostumeRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load a user's costume state, returning None if no row exists.
    pub async fn load(&self, character_id: &str) -> Result<Option<UserCostume>, sqlx::Error> {
        sqlx::query_as::<_, UserCostume>(
            "SELECT character_id, active_type, item_id, item_param, scale_raw, \
             color_index, expiry_time \
             FROM user_costume WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save the full costume state for a character (upsert).
    #[allow(clippy::too_many_arguments)]
    pub async fn save(
        &self,
        character_id: &str,
        active_type: i16,
        item_id: i32,
        item_param: i32,
        scale_raw: i32,
        color_index: i16,
        expiry_time: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_costume \
             (character_id, active_type, item_id, item_param, scale_raw, color_index, expiry_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             ON CONFLICT (character_id) DO UPDATE SET \
             active_type = $2, item_id = $3, item_param = $4, \
             scale_raw = $5, color_index = $6, expiry_time = $7",
        )
        .bind(character_id)
        .bind(active_type)
        .bind(item_id)
        .bind(item_param)
        .bind(scale_raw)
        .bind(color_index)
        .bind(expiry_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
