//! JackPot setting models — maps to `jackpot_settings` table.
//! Two rows: iType 0 = EXP jackpot, iType 1 = Noah/gold jackpot.

/// A jackpot setting row loaded at startup.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct JackPotSettingRow {
    /// 0 = EXP, 1 = Noah.
    pub i_type: i16,
    /// Chance out of 10000 that jackpot triggers at all.
    pub rate: i16,
    /// Threshold for 1000x multiplier (rand < x_1000 → 1000x).
    pub x_1000: i16,
    /// Threshold for 500x multiplier.
    pub x_500: i16,
    /// Threshold for 100x multiplier.
    pub x_100: i16,
    /// Threshold for 50x multiplier.
    pub x_50: i16,
    /// Threshold for 10x multiplier.
    pub x_10: i16,
    /// Threshold for 2x multiplier.
    pub x_2: i16,
}
