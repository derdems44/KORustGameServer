//! Hermetic Seal model — maps to `user_hermetic_seal` PostgreSQL table.
//!
//! v2525 WIZ_ABILITY (0xCF) — 24-slot wheel with 9 upgrade levels.

/// A row from the `user_hermetic_seal` table — per-user Hermetic Seal state.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserHermeticSeal {
    /// Character name (primary key).
    pub character_id: String,
    /// Maximum tier achieved (0-9).
    pub max_tier: i16,
    /// Currently selected slot index (0-23).
    pub selected_slot: i16,
    /// Status: 0=active, 1=paused, 2=completed.
    pub status: i16,
    /// Number of upgrade attempts.
    pub upgrade_count: i16,
    /// Current upgrade level (0-9).
    pub current_level: i16,
    /// Elapsed progress time in seconds.
    pub elapsed_time: f32,
}
