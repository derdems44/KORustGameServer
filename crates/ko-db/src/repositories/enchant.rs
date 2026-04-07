//! Enchant repository — user enchant state persistence.
//!
//! v2525 WIZ_ENCHANT (0xCC) — weapon/armor + item enchantment.

use crate::models::enchant::UserEnchant;
use crate::DbPool;

/// Repository for user_enchant table access.
pub struct EnchantRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> EnchantRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load a user's enchant state, returning None if no row exists.
    pub async fn load(&self, character_id: &str) -> Result<Option<UserEnchant>, sqlx::Error> {
        sqlx::query_as::<_, UserEnchant>(
            "SELECT character_id, max_star, enchant_count, slot_levels, slot_unlocked, \
             item_category, item_slot_unlock, item_markers \
             FROM user_enchant WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save the full enchant state for a character (upsert).
    #[allow(clippy::too_many_arguments)]
    pub async fn save(
        &self,
        character_id: &str,
        max_star: i16,
        enchant_count: i16,
        slot_levels: &[u8],
        slot_unlocked: &[u8],
        item_category: i16,
        item_slot_unlock: i16,
        item_markers: &[u8],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_enchant \
             (character_id, max_star, enchant_count, slot_levels, slot_unlocked, \
              item_category, item_slot_unlock, item_markers) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
             ON CONFLICT (character_id) DO UPDATE SET \
             max_star = $2, enchant_count = $3, slot_levels = $4, slot_unlocked = $5, \
             item_category = $6, item_slot_unlock = $7, item_markers = $8",
        )
        .bind(character_id)
        .bind(max_star)
        .bind(enchant_count)
        .bind(slot_levels)
        .bind(slot_unlocked)
        .bind(item_category)
        .bind(item_slot_unlock)
        .bind(item_markers)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
