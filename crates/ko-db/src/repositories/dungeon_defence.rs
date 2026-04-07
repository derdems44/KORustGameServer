//! Dungeon Defence (Full Moon Rift) stage and monster spawn repository.
//!                `m_DungeonDefenceMonsterListArray` at server startup.

use crate::models::dungeon_defence::{DfMonsterRow, DfStageRow};
use crate::DbPool;

/// Repository for Dungeon Defence stage/monster data.
pub struct DungeonDefenceRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> DungeonDefenceRepository<'a> {
    /// Create a new dungeon defence repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all DD stage definitions from the database.
    ///
    pub async fn load_all_stages(&self) -> Result<Vec<DfStageRow>, sqlx::Error> {
        sqlx::query_as::<_, DfStageRow>(
            "SELECT id, difficulty, difficulty_name, stage_id \
             FROM df_stage_list ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all DD monster spawn definitions from the database.
    ///
    pub async fn load_all_monsters(&self) -> Result<Vec<DfMonsterRow>, sqlx::Error> {
        sqlx::query_as::<_, DfMonsterRow>(
            "SELECT id, difficulty, monster_id, is_monster, pos_x, pos_z, \
             s_count, s_direction, s_radius_range \
             FROM df_monster_list ORDER BY id",
        )
        .fetch_all(self.pool)
        .await
    }
}
