//! NPC repository — loads NPC/Monster templates and spawn data from PostgreSQL.
//! - `GameServer/LoadServerData.cpp` — LoadNpcTableData(), LoadNpcPosTable()

use crate::models::{
    MonsterBossRandomSpawnRow, MonsterRespawnLoopRow, MonsterSummonRow, NpcSpawnRow, NpcTemplateRow,
};
use crate::DbPool;

/// Repository for `npc_template` and `npc_spawn` table access.
pub struct NpcRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> NpcRepository<'a> {
    /// Create a new repository backed by the given connection pool.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all NPC/Monster templates (bulk load at startup).
    ///
    pub async fn load_all_templates(&self) -> Result<Vec<NpcTemplateRow>, sqlx::Error> {
        sqlx::query_as::<_, NpcTemplateRow>(
            "SELECT s_sid, is_monster, str_name, s_pid, s_size, \
             i_weapon_1, i_weapon_2, by_group, by_act_type, by_type, \
             by_family, by_rank, by_title, i_selling_group, s_level, \
             i_exp, i_loyalty, i_hp_point, s_mp_point, s_atk, s_ac, \
             s_hit_rate, s_evade_rate, s_damage, s_attack_delay, \
             by_speed_1, by_speed_2, s_standtime, s_item, \
             i_magic_1, i_magic_2, i_magic_3, \
             s_fire_r, s_cold_r, s_lightning_r, s_magic_r, s_disease_r, s_poison_r, \
             s_bulk, by_attack_range, by_search_range, by_tracing_range, \
             i_money, by_direct_attack, by_magic_attack, area_range \
             FROM npc_template ORDER BY s_sid, is_monster",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all NPC spawn positions (bulk load at startup).
    ///
    pub async fn load_all_spawns(&self) -> Result<Vec<NpcSpawnRow>, sqlx::Error> {
        sqlx::query_as::<_, NpcSpawnRow>(
            "SELECT id, zone_id, npc_id, is_monster, act_type, regen_type, \
             dungeon_family, special_type, trap_number, \
             left_x, top_z, num_npc, spawn_range, regen_time, \
             direction, dot_cnt, path, room \
             FROM npc_spawn ORDER BY zone_id, id",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all monster summon list entries (bulk load at startup).
    ///
    pub async fn load_monster_summon_list(&self) -> Result<Vec<MonsterSummonRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterSummonRow>(
            "SELECT s_sid, str_name, s_level, s_probability, b_type \
             FROM monster_summon_list ORDER BY s_sid",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all monster respawn loop entries (bulk load at startup).
    ///
    pub async fn load_monster_respawn_loop(
        &self,
    ) -> Result<Vec<MonsterRespawnLoopRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterRespawnLoopRow>(
            "SELECT idead, iborn, stable, count, deadtime \
             FROM monster_respawn_loop ORDER BY idead",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all boss random spawn entries (bulk load at startup).
    ///
    pub async fn load_boss_random_spawn(
        &self,
    ) -> Result<Vec<MonsterBossRandomSpawnRow>, sqlx::Error> {
        sqlx::query_as::<_, MonsterBossRandomSpawnRow>(
            "SELECT n_index, stage, monster_id, monster_zone, pos_x, pos_z, \
             range, reload_time, monster_name \
             FROM monster_boss_random_spawn ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }
}
