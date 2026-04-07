//! Monster event spawn models.
//!                `MONSTER_JURAID_MOUNTAIN_RESPAWN_LIST`, `MONSTER_CHALLENGE`,
//!                `MONSTER_CHALLENGE_SUMMON_LIST` tables in MSSQL.

/// A row from `monster_stone_respawn_list` -- stone dungeon spawn points.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterStoneRespawnRow {
    /// Primary key index.
    pub s_index: i16,
    /// NPC template SID.
    pub s_sid: i16,
    /// NPC type (0=monster, 1=NPC).
    pub b_type: i16,
    /// Monster/NPC display name.
    pub str_name: String,
    /// Prototype ID.
    pub s_pid: i16,
    /// Zone ID (81 for Monster Stone).
    pub zone_id: i16,
    /// Whether this is a boss (1=boss).
    pub is_boss: bool,
    /// Family grouping (1-26).
    pub family: i16,
    /// Spawn count.
    pub s_count: i16,
    /// Facing direction.
    pub by_direction: i16,
    /// X coordinate.
    pub x: i16,
    /// Y coordinate.
    pub y: i16,
    /// Z coordinate.
    pub z: i16,
}

/// A row from `monster_boss_random_stages` -- random boss spawn stage config.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterBossRandomStageRow {
    /// Stage number (1-33).
    pub stage: i16,
    /// Monster template ID to spawn.
    pub monster_id: i16,
    /// Zone where the boss spawns.
    pub monster_zone: i16,
    /// Monster display name.
    pub monster_name: String,
}

/// A row from `monster_juraid_respawn_list` -- Juraid Mountain event spawns.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterJuraidRespawnRow {
    /// Primary key index.
    pub s_index: i16,
    /// NPC template SID.
    pub s_sid: i16,
    /// NPC type (0=monster, 1=NPC).
    pub b_type: i16,
    /// Monster/NPC display name.
    pub str_name: String,
    /// Prototype ID.
    pub s_pid: i16,
    /// Zone ID (87 for Juraid Mountain).
    pub zone_id: i16,
    /// Family grouping (21-28).
    pub family: i16,
    /// Spawn count.
    pub s_count: i16,
    /// X coordinate.
    pub x: i16,
    /// Y coordinate.
    pub y: i16,
    /// Z coordinate.
    pub z: i16,
    /// Facing direction.
    pub by_direction: i16,
    /// Spawn radius.
    pub b_radius: Option<i16>,
}

/// A row from `monster_challenge` -- challenge event config per level bracket.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterChallengeRow {
    /// Config index (0-2).
    pub s_index: i16,
    /// First start hour (99=disabled).
    pub b_start_time1: i16,
    /// Second start hour (99=disabled).
    pub b_start_time2: i16,
    /// Third start hour (99=disabled).
    pub b_start_time3: i16,
    /// Minimum player level.
    pub b_level_min: i16,
    /// Maximum player level.
    pub b_level_max: i16,
}

/// A row from `monster_challenge_summon_list` -- challenge wave definitions.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterChallengeSummonRow {
    /// Primary key index.
    pub s_index: i16,
    /// Difficulty level (1-3).
    pub b_level: i16,
    /// Stage number within the level.
    pub b_stage: i16,
    /// Sub-stage difficulty.
    pub b_stage_level: i16,
    /// Time offset for this wave (seconds from start).
    pub s_time: i16,
    /// Monster template SID.
    pub s_sid: i16,
    /// Monster display name.
    pub str_name: Option<String>,
    /// Number of monsters to spawn.
    pub s_count: i16,
    /// Spawn X position.
    pub s_pos_x: i16,
    /// Spawn Z position.
    pub s_pos_z: i16,
    /// Spawn radius range.
    pub b_range: i16,
}
