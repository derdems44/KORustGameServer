//! Dungeon Defence (Full Moon Rift) stage and monster spawn models.
//!                in `GameDefine.h`

/// A row from the `df_stage_list` table -- maps difficulty to stage IDs.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DfStageRow {
    /// Primary key.
    pub id: i32,
    /// Difficulty level (1=Easy, 2=Normal, 3=Hard).
    pub difficulty: i16,
    /// Human-readable difficulty name.
    pub difficulty_name: Option<String>,
    /// Stage ID within this difficulty tier.
    pub stage_id: i16,
}

/// A row from the `df_monster_list` table -- monster spawns per stage.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DfMonsterRow {
    /// Stage ID (matches `df_stage_list.stage_id` for lookup).
    pub id: i32,
    /// Difficulty level (1=Easy, 2=Normal, 3=Hard).
    pub difficulty: Option<i16>,
    /// NPC template ID for the monster to spawn.
    pub monster_id: i16,
    /// Whether this is a combat monster (0) or NPC (1).
    pub is_monster: bool,
    /// X coordinate for spawn position.
    pub pos_x: i16,
    /// Z coordinate for spawn position.
    pub pos_z: i16,
    /// Number of monsters to spawn.
    pub s_count: Option<i16>,
    /// Facing direction.
    pub s_direction: i16,
    /// Random spawn radius around the position.
    pub s_radius_range: Option<i16>,
}
