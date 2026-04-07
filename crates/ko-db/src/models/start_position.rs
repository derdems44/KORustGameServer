//! Start position models — per-zone spawn/respawn coordinates.
//! Source: MSSQL `START_POSITION2369` (79 rows), `START_POSITION_RANDOM` (38 rows)

/// Per-zone spawn/respawn coordinates for Karus and El Morad nations.
/// Used for death respawn, `/town` command, zone change default coords.
/// Keyed by `zone_id`. Each row has separate X/Z coords per nation,
/// optional gate coords, and a random range offset.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StartPositionRow {
    /// Zone identifier (PK).
    pub zone_id: i16,
    /// Karus nation spawn X coordinate.
    pub karus_x: i16,
    /// Karus nation spawn Z coordinate.
    pub karus_z: i16,
    /// El Morad nation spawn X coordinate.
    pub elmorad_x: i16,
    /// El Morad nation spawn Z coordinate.
    pub elmorad_z: i16,
    /// Karus gate X coordinate (special zones).
    pub karus_gate_x: i16,
    /// Karus gate Z coordinate (special zones).
    pub karus_gate_z: i16,
    /// El Morad gate X coordinate (special zones).
    pub elmo_gate_x: i16,
    /// El Morad gate Z coordinate (special zones).
    pub elmo_gate_z: i16,
    /// Random offset range in X (spawn at x + rand(0..range_x)).
    pub range_x: i16,
    /// Random offset range in Z (spawn at z + rand(0..range_z)).
    pub range_z: i16,
}

/// Random spawn point for special zones (Chaos Dungeon, Bowl events).
/// Multiple points per zone; server picks one at random and applies radius offset.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StartPositionRandomRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Zone identifier.
    pub zone_id: i16,
    /// Spawn X coordinate.
    pub pos_x: i16,
    /// Spawn Z coordinate.
    pub pos_z: i16,
    /// Random radius offset applied to pos_x/pos_z.
    pub radius: i16,
}
