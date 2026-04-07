//! Level-up experience table model.
//! Maps to the `level_up` PostgreSQL table: required XP per level.

/// A single row from the `level_up` table.
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
