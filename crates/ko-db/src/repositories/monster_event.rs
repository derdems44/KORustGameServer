//! Monster event data repository.

use crate::models::monster_event::{
    MonsterBossRandomStageRow, MonsterChallengeRow, MonsterChallengeSummonRow,
    MonsterJuraidRespawnRow, MonsterStoneRespawnRow,
};
use crate::DbPool;

/// Repository for monster event spawn data.
pub struct MonsterEventRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> MonsterEventRepository<'a> {
    /// Create a new monster event repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all monster stone respawn entries.
    ///
    pub async fn load_stone_respawn(&self) -> Result<Vec<MonsterStoneRespawnRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterStoneRespawnRow>(
            "SELECT s_index, s_sid, b_type, str_name, s_pid, zone_id, is_boss, \
             family, s_count, by_direction, x, y, z \
             FROM monster_stone_respawn_list ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all boss random stage definitions.
    ///
    pub async fn load_boss_random_stages(
        &self,
    ) -> Result<Vec<MonsterBossRandomStageRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterBossRandomStageRow>(
            "SELECT stage, monster_id, monster_zone, monster_name \
             FROM monster_boss_random_stages ORDER BY stage",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all Juraid Mountain respawn entries.
    ///
    pub async fn load_juraid_respawn(&self) -> Result<Vec<MonsterJuraidRespawnRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterJuraidRespawnRow>(
            "SELECT s_index, s_sid, b_type, str_name, s_pid, zone_id, family, \
             s_count, x, y, z, by_direction, b_radius \
             FROM monster_juraid_respawn_list ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all monster challenge config entries.
    ///
    pub async fn load_challenge_config(&self) -> Result<Vec<MonsterChallengeRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterChallengeRow>(
            "SELECT s_index, b_start_time1, b_start_time2, b_start_time3, \
             b_level_min, b_level_max \
             FROM monster_challenge ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all monster challenge summon list entries.
    ///
    pub async fn load_challenge_summon_list(
        &self,
    ) -> Result<Vec<MonsterChallengeSummonRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterChallengeSummonRow>(
            "SELECT s_index, b_level, b_stage, b_stage_level, s_time, s_sid, \
             str_name, s_count, s_pos_x, s_pos_z, b_range \
             FROM monster_challenge_summon_list ORDER BY s_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
