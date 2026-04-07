//! Extended item upgrade models — maps to PostgreSQL tables.
//! - `shared/database/ItemUpgradeSet.h` — `_ITEMUP_PROBABILITY`
//! - `GameServer/LoadServerData.cpp` — `LoadItemUpProbability()`

/// Item upgrade probability configuration from the `itemup_probability` table.
/// Controls upgrade success/fail streak-based probability modifiers.
/// MSSQL source: `ITEMUP_PROBABILITY` (1 row).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemUpProbabilityRow {
    /// Probability configuration type.
    pub b_type: i16,
    /// Maximum success streak counter.
    pub max_success: i16,
    /// Maximum fail streak counter.
    pub max_fail: i16,
    /// Current success streak counter.
    pub cur_success: i16,
    /// Current fail streak counter.
    pub cur_fail: i16,
}
