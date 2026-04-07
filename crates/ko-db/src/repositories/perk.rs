//! Perk repository — `perks` and `user_perks` table access.
//!
//! C++ Reference: `CDBAgent::LoadPerksData`, `CDBAgent::UpdateUserPerks`
//! in `DBAgent.cpp:5698-5745`.

use sqlx::PgPool;

use crate::models::perk::{PerkRow, UserPerkRow, PERK_COUNT};

/// Repository for perk system database operations.
pub struct PerkRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PerkRepository<'a> {
    /// Create a new perk repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load all perk definitions (static data, loaded once at startup).
    ///
    /// C++ Reference: `CGameServerDlg::m_PerksArray` loaded from MSSQL PERKS table.
    pub async fn load_all_perks(&self) -> Result<Vec<PerkRow>, sqlx::Error> {
        sqlx::query_as::<_, PerkRow>(
            "SELECT p_index, status, description, perk_count, perk_max, percentage \
             FROM perks \
             ORDER BY p_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load a character's perk allocations.
    ///
    /// Returns None if the character has no perk record yet.
    ///
    /// C++ Reference: `CDBAgent::LoadPerksData` — `{CALL LOAD_PERKS_DATA(?)}`
    pub async fn load_user_perks(
        &self,
        character_id: &str,
    ) -> Result<Option<UserPerkRow>, sqlx::Error> {
        sqlx::query_as::<_, UserPerkRow>(
            "SELECT character_id, \
                    perk_type0, perk_type1, perk_type2, perk_type3, \
                    perk_type4, perk_type5, perk_type6, perk_type7, \
                    perk_type8, perk_type9, perk_type10, perk_type11, \
                    perk_type12, rem_perk \
             FROM user_perks \
             WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save (upsert) a character's perk allocations.
    ///
    /// C++ Reference: `CDBAgent::UpdateUserPerks` — `{CALL UPDATE_USER_PERKS(...)}`
    pub async fn save_user_perks(
        &self,
        character_id: &str,
        perk_levels: &[i16; PERK_COUNT],
        rem_perk: i16,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_perks \
                (character_id, perk_type0, perk_type1, perk_type2, perk_type3, \
                 perk_type4, perk_type5, perk_type6, perk_type7, \
                 perk_type8, perk_type9, perk_type10, perk_type11, \
                 perk_type12, rem_perk) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) \
             ON CONFLICT (character_id) DO UPDATE SET \
                perk_type0 = $2, perk_type1 = $3, perk_type2 = $4, perk_type3 = $5, \
                perk_type4 = $6, perk_type5 = $7, perk_type6 = $8, perk_type7 = $9, \
                perk_type8 = $10, perk_type9 = $11, perk_type10 = $12, perk_type11 = $13, \
                perk_type12 = $14, rem_perk = $15",
        )
        .bind(character_id)
        .bind(perk_levels[0])
        .bind(perk_levels[1])
        .bind(perk_levels[2])
        .bind(perk_levels[3])
        .bind(perk_levels[4])
        .bind(perk_levels[5])
        .bind(perk_levels[6])
        .bind(perk_levels[7])
        .bind(perk_levels[8])
        .bind(perk_levels[9])
        .bind(perk_levels[10])
        .bind(perk_levels[11])
        .bind(perk_levels[12])
        .bind(rem_perk)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Delete a character's perk record (e.g., on character delete).
    pub async fn delete_user_perks(&self, character_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_perks WHERE character_id = $1")
            .bind(character_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
