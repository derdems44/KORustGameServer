//! Hermetic Seal repository — user ability/seal state persistence.
//!
//! v2525 WIZ_ABILITY (0xCF) — 24-slot wheel with 9 upgrade levels.

use crate::models::hermetic_seal::UserHermeticSeal;
use crate::DbPool;

/// Repository for hermetic seal table access.
pub struct HermeticSealRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> HermeticSealRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load a user's Hermetic Seal state, creating default if missing.
    pub async fn load_or_create(
        &self,
        character_id: &str,
    ) -> Result<UserHermeticSeal, sqlx::Error> {
        let row = sqlx::query_as::<_, UserHermeticSeal>(
            "SELECT character_id, max_tier, selected_slot, status, upgrade_count, \
             current_level, elapsed_time \
             FROM user_hermetic_seal WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await?;

        if let Some(r) = row {
            return Ok(r);
        }

        // Insert default row
        sqlx::query(
            "INSERT INTO user_hermetic_seal (character_id) VALUES ($1) ON CONFLICT DO NOTHING",
        )
        .bind(character_id)
        .execute(self.pool)
        .await?;

        Ok(UserHermeticSeal {
            character_id: character_id.to_string(),
            max_tier: 0,
            selected_slot: 0,
            status: 1,
            upgrade_count: 0,
            current_level: 0,
            elapsed_time: 0.0,
        })
    }

    /// Save the full Hermetic Seal state for a character.
    #[allow(clippy::too_many_arguments)]
    pub async fn save(
        &self,
        character_id: &str,
        max_tier: i16,
        selected_slot: i16,
        status: i16,
        upgrade_count: i16,
        current_level: i16,
        elapsed_time: f32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_hermetic_seal \
             (character_id, max_tier, selected_slot, status, upgrade_count, current_level, elapsed_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             ON CONFLICT (character_id) DO UPDATE SET \
             max_tier = $2, selected_slot = $3, status = $4, \
             upgrade_count = $5, current_level = $6, elapsed_time = $7",
        )
        .bind(character_id)
        .bind(max_tier)
        .bind(selected_slot)
        .bind(status)
        .bind(upgrade_count)
        .bind(current_level)
        .bind(elapsed_time)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Update only the selected slot (for quick slot selection saves).
    pub async fn update_selected_slot(
        &self,
        character_id: &str,
        selected_slot: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE user_hermetic_seal SET selected_slot = $1 WHERE character_id = $2")
            .bind(selected_slot)
            .bind(character_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
