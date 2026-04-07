//! Level-up experience table model.
//!
//! C++ Reference: `shared/database/LevelUpTableSet.h`
//!
//! Maps to the `level_up` PostgreSQL table: required XP per level.

/// A single row from the `level_up` table.
///
/// C++ Reference: `_LEVEL_UP` struct
/// - `Level`: the character level
/// - `Exp`: XP required to level up from this level
/// - `RebithLevel`: rebirth tier (0 = normal, 1-10 = rebirth)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct LevelUpRow {
    pub id: i16,
    pub level: i16,
    pub exp: i64,
    pub rebirth_level: i16,
}
