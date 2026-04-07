//! Soul system repository — load/save per-character soul data.
//!
//! v2525-specific: WIZ_SOUL (0xC5) panel persistence.

use sqlx::PgPool;

use crate::models::soul::UserSoulDataRow;

/// Repository for soul data persistence.
pub struct SoulRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SoulRepository<'a> {
    /// Create a new soul repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load soul data for a character.
    pub async fn load(&self, character_id: &str) -> Result<Option<UserSoulDataRow>, sqlx::Error> {
        sqlx::query_as::<_, UserSoulDataRow>(
            "SELECT character_id, \
             cat0_v0, cat0_v1, cat0_v2, cat1_v0, cat1_v1, cat1_v2, \
             cat2_v0, cat2_v1, cat2_v2, cat3_v0, cat3_v1, cat3_v2, \
             cat4_v0, cat4_v1, cat4_v2, cat5_v0, cat5_v1, cat5_v2, \
             cat6_v0, cat6_v1, cat6_v2, cat7_v0, cat7_v1, cat7_v2, \
             slot0, slot1, slot2, slot3, slot4 \
             FROM user_soul_data WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save (upsert) soul data for a character.
    pub async fn save(
        &self,
        character_id: &str,
        categories: &[[i16; 4]; 8],
        slots: &[[i16; 2]; 5],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_soul_data (character_id, \
             cat0_v0, cat0_v1, cat0_v2, cat1_v0, cat1_v1, cat1_v2, \
             cat2_v0, cat2_v1, cat2_v2, cat3_v0, cat3_v1, cat3_v2, \
             cat4_v0, cat4_v1, cat4_v2, cat5_v0, cat5_v1, cat5_v2, \
             cat6_v0, cat6_v1, cat6_v2, cat7_v0, cat7_v1, cat7_v2, \
             slot0, slot1, slot2, slot3, slot4) \
             VALUES ($1, \
             $2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13, \
             $14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25, \
             $26,$27,$28,$29,$30) \
             ON CONFLICT (character_id) DO UPDATE SET \
             cat0_v0=EXCLUDED.cat0_v0, cat0_v1=EXCLUDED.cat0_v1, cat0_v2=EXCLUDED.cat0_v2, \
             cat1_v0=EXCLUDED.cat1_v0, cat1_v1=EXCLUDED.cat1_v1, cat1_v2=EXCLUDED.cat1_v2, \
             cat2_v0=EXCLUDED.cat2_v0, cat2_v1=EXCLUDED.cat2_v1, cat2_v2=EXCLUDED.cat2_v2, \
             cat3_v0=EXCLUDED.cat3_v0, cat3_v1=EXCLUDED.cat3_v1, cat3_v2=EXCLUDED.cat3_v2, \
             cat4_v0=EXCLUDED.cat4_v0, cat4_v1=EXCLUDED.cat4_v1, cat4_v2=EXCLUDED.cat4_v2, \
             cat5_v0=EXCLUDED.cat5_v0, cat5_v1=EXCLUDED.cat5_v1, cat5_v2=EXCLUDED.cat5_v2, \
             cat6_v0=EXCLUDED.cat6_v0, cat6_v1=EXCLUDED.cat6_v1, cat6_v2=EXCLUDED.cat6_v2, \
             cat7_v0=EXCLUDED.cat7_v0, cat7_v1=EXCLUDED.cat7_v1, cat7_v2=EXCLUDED.cat7_v2, \
             slot0=EXCLUDED.slot0, slot1=EXCLUDED.slot1, slot2=EXCLUDED.slot2, \
             slot3=EXCLUDED.slot3, slot4=EXCLUDED.slot4",
        )
        .bind(character_id)
        .bind(categories[0][1])
        .bind(categories[0][2])
        .bind(categories[0][3])
        .bind(categories[1][1])
        .bind(categories[1][2])
        .bind(categories[1][3])
        .bind(categories[2][1])
        .bind(categories[2][2])
        .bind(categories[2][3])
        .bind(categories[3][1])
        .bind(categories[3][2])
        .bind(categories[3][3])
        .bind(categories[4][1])
        .bind(categories[4][2])
        .bind(categories[4][3])
        .bind(categories[5][1])
        .bind(categories[5][2])
        .bind(categories[5][3])
        .bind(categories[6][1])
        .bind(categories[6][2])
        .bind(categories[6][3])
        .bind(categories[7][1])
        .bind(categories[7][2])
        .bind(categories[7][3])
        .bind(slots[0][1])
        .bind(slots[1][1])
        .bind(slots[2][1])
        .bind(slots[3][1])
        .bind(slots[4][1])
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
