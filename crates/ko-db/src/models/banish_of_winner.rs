//! Banish of winner model — maps to the `banish_of_winner` PostgreSQL table.
//!
//! Source: MSSQL `BANISH_OF_WINNER` table — post-war monster spawn definitions.

/// A row from the `banish_of_winner` table — defines monster spawns
/// that appear after a war victory (Banish event).
///
/// Each row specifies a spawn location, nation filter, and respawn parameters
/// for monsters summoned during the banish-of-winner event.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BanishOfWinner {
    /// Primary key index.
    pub idx: i32,
    /// NPC/monster template SID.
    pub sid: i16,
    /// Nation filter (0=both, 1=Karus, 2=Elmorad). NULL means no restriction.
    pub nation_id: Option<i16>,
    /// Zone ID where the monster spawns.
    pub zone_id: i16,
    /// X coordinate of the spawn point.
    pub pos_x: i16,
    /// Z coordinate of the spawn point.
    pub pos_z: i16,
    /// Number of monsters to spawn at this point.
    pub spawn_count: i16,
    /// Spawn radius around the position. NULL means point-spawn.
    pub radius: Option<i16>,
    /// Time in seconds before the monster despawns after death.
    pub dead_time: i16,
}
