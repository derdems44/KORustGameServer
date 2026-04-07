//! Burning features model — maps to the `burning_features` PostgreSQL table.
//!
//! Source: MSSQL `BURNING_FEATURES` table — per-level rate multipliers.

/// A row from the `burning_features` table — rate multipliers by level range.
///
/// Each row defines bonus rates for a specific level tier. The server applies
/// the matching tier's multipliers to NP, money, EXP, and drop calculations.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BurningFeatures {
    /// Level threshold for this burning tier.
    pub burn_level: i16,
    /// Nation Point (NP/loyalty) rate multiplier (percentage).
    pub np_rate: i16,
    /// Gold/money rate multiplier (percentage).
    pub money_rate: i16,
    /// Experience rate multiplier (percentage).
    pub exp_rate: i16,
    /// Item drop rate multiplier (percentage).
    pub drop_rate: i16,
}
