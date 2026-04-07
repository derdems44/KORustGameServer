//! Zone info model — maps to the `zone_info` PostgreSQL table.
//!
//! C++ Reference: `shared/database/structs.h` — `_ZONE_INFO`

/// A single zone configuration row from the database.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ZoneInfoRow {
    pub zone_no: i16,
    pub smd_name: String,
    pub zone_name: String,
    pub zone_type: i16,
    pub min_level: i16,
    pub max_level: i16,
    pub init_x: i32,
    pub init_z: i32,
    pub init_y: i32,
    pub trade_other_nation: bool,
    pub talk_other_nation: bool,
    pub attack_other_nation: bool,
    pub attack_same_nation: bool,
    pub friendly_npc: bool,
    pub war_zone: bool,
    pub clan_updates: bool,
    pub teleport: bool,
    pub gate: bool,
    pub escape: bool,
    pub calling_friend: bool,
    pub teleport_friend: bool,
    pub blink: bool,
    pub pet_spawn: bool,
    pub exp_lost: bool,
    pub give_loyalty: bool,
    pub guard_summon: bool,
    pub military_zone: bool,
    pub mining_zone: bool,
    pub blink_zone: bool,
    pub auto_loot: bool,
    pub gold_lose: bool,
    /// Zone status (C++ `m_Status`). 0=inactive, 1=active.
    pub status: i16,
}
