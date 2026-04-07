//! NPC/Monster model structs — maps to `npc_template` and `npc_spawn` PostgreSQL tables.
//!
//! C++ Reference:
//! - `shared/database/NpcTableSet.h` — K_NPC / K_MONSTER table loader
//! - `shared/database/NpcPosSet.h` — K_NPCPOS spawn position loader

/// A single NPC/Monster template row from the database.
///
/// Represents the static data for an NPC or monster type (stats, appearance, etc.).
/// K_NPC and K_MONSTER share the same 45-column schema; `is_monster` distinguishes them.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcTemplateRow {
    pub s_sid: i16,
    pub is_monster: bool,
    pub str_name: Option<String>,
    pub s_pid: i16,
    pub s_size: i16,
    pub i_weapon_1: i32,
    pub i_weapon_2: i32,
    pub by_group: i16,
    pub by_act_type: i16,
    pub by_type: i16,
    pub by_family: i16,
    pub by_rank: i16,
    pub by_title: i16,
    pub i_selling_group: i32,
    pub s_level: i16,
    pub i_exp: i32,
    pub i_loyalty: i32,
    pub i_hp_point: i32,
    pub s_mp_point: i16,
    pub s_atk: i16,
    pub s_ac: i16,
    pub s_hit_rate: i16,
    pub s_evade_rate: i16,
    pub s_damage: i16,
    pub s_attack_delay: i16,
    pub by_speed_1: i16,
    pub by_speed_2: i16,
    pub s_standtime: i16,
    pub s_item: i16,
    pub i_magic_1: i32,
    pub i_magic_2: i32,
    pub i_magic_3: i32,
    pub s_fire_r: i16,
    pub s_cold_r: i16,
    pub s_lightning_r: i16,
    pub s_magic_r: i16,
    pub s_disease_r: i16,
    pub s_poison_r: i16,
    pub s_bulk: i16,
    pub by_attack_range: i16,
    pub by_search_range: i16,
    pub by_tracing_range: i16,
    pub i_money: i32,
    pub by_direct_attack: i16,
    pub by_magic_attack: i16,
    pub area_range: f32,
}

/// A monster summon list entry — defines summonable monsters from scrolls/stones.
///
/// C++ Reference: `MONSTER_SUMMON_LIST` table
/// bType: 1 = standard summon, 2 = special summon
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterSummonRow {
    pub s_sid: i16,
    pub str_name: String,
    pub s_level: i16,
    pub s_probability: i16,
    pub b_type: i16,
}

/// A monster respawn loop entry — chain respawn when a monster dies.
///
/// C++ Reference: `MONSTER_RESPAWNLOOP_LIST` table
/// When monster `idead` dies, monster `iborn` spawns after `deadtime` seconds.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterRespawnLoopRow {
    pub idead: i16,
    pub iborn: i16,
    pub stable: bool,
    pub count: i16,
    pub deadtime: i16,
}

/// A boss random spawn pool entry — candidate positions for timed boss spawns.
///
/// C++ Reference: `MONSTER_BOSS_RANDOM_SPAWN` table
/// Multiple entries per stage; a random one is picked for each spawn cycle.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterBossRandomSpawnRow {
    pub n_index: i32,
    pub stage: i32,
    pub monster_id: i32,
    pub monster_zone: i32,
    pub pos_x: i32,
    pub pos_z: i32,
    pub range: i32,
    pub reload_time: i32,
    pub monster_name: String,
}

/// A single NPC spawn position row from the database.
///
/// Defines where and how many NPCs/Monsters should spawn in a zone.
/// Coordinates are raw world units (NOT multiplied by 100).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcSpawnRow {
    pub id: i32,
    pub zone_id: i16,
    pub npc_id: i16,
    pub is_monster: bool,
    pub act_type: i16,
    pub regen_type: i16,
    pub dungeon_family: i16,
    pub special_type: i16,
    pub trap_number: i16,
    pub left_x: i32,
    pub top_z: i32,
    pub num_npc: i16,
    pub spawn_range: i16,
    pub regen_time: i16,
    pub direction: i32,
    pub dot_cnt: i16,
    pub path: Option<String>,
    pub room: i16,
}
