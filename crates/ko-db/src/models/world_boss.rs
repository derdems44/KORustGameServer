//! World Boss data models — boss configuration and ranking.

/// World boss slot configuration.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WorldBossConfigRow {
    /// Panel slot (1-4).
    pub slot_id: i16,
    /// Boss display name.
    pub boss_name: String,
    /// NPC template ID for spawning.
    pub npc_proto_id: i32,
    /// Boss type (1-4, determines gauge clamping).
    pub boss_type: i16,
    /// Animation resource lookup ID.
    pub boss_info_id: i16,
    /// Zone ID for boss spawn.
    pub spawn_zone: i16,
    /// Spawn X coordinate.
    pub spawn_x: f32,
    /// Spawn Z coordinate.
    pub spawn_z: f32,
    /// Whether this slot is active.
    pub enabled: bool,
}

/// Per-event player ranking entry.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WorldBossRankingRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Boss slot (1-4).
    pub slot_id: i16,
    /// Player character name.
    pub character_id: String,
    /// Total damage dealt in this event.
    pub damage_dealt: i64,
    /// Times participated in kill.
    pub kill_count: i32,
    /// Whether this player got the last hit.
    pub last_hit: bool,
}
