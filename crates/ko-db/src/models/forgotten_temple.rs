//! Forgotten Temple (Monster Challenge) stage and summon models.
//!                in `GameDefine.h`
//! FT event options (`EventOptFtRow`) and rewards (`EventRewardRow`) are defined
//! in the shared `event_schedule` module.

/// A row from the `ft_stages` table -- defines timing for each stage.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FtStageRow {
    /// Primary key index.
    pub n_index: i32,
    /// Event type (1 = standard Monster Challenge).
    pub event_type: i16,
    /// Stage number (1-60).
    pub stage: i16,
    /// Time offset in seconds from summon start when this stage triggers.
    pub time_offset: i16,
}

/// A row from the `ft_summon_list` table -- defines monsters to spawn per stage.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct FtSummonRow {
    /// Row index (not unique -- MSSQL has duplicate bIndex values).
    pub b_index: i32,
    /// Event type (1 = standard Monster Challenge).
    pub event_type: i16,
    /// Stage number this spawn belongs to.
    pub stage: i16,
    /// NPC template ID to spawn.
    pub sid_id: i16,
    /// Number of NPCs to spawn at this point.
    pub sid_count: i16,
    /// X coordinate for spawn position.
    pub pos_x: i16,
    /// Z coordinate for spawn position.
    pub pos_z: i16,
    /// Spawn range (radius around pos_x/pos_z).
    pub spawn_range: i16,
    /// Display name of the summoned monster.
    pub summon_name: String,
}
