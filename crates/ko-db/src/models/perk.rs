//! Perk system models — maps to `perks` and `user_perks` tables.
//! MSSQL Reference: PERKS (13 definitions), USER_PERKS (per-character allocations).

/// Total number of perk types.
pub const PERK_COUNT: usize = 13;

/// A perk definition row (static data loaded at startup).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PerkRow {
    /// Perk index (0-12).
    pub p_index: i32,
    /// Whether this perk is enabled.
    pub status: bool,
    /// Human-readable description.
    pub description: String,
    /// Bonus amount per level.
    pub perk_count: i16,
    /// Maximum number of levels for this perk.
    pub perk_max: i16,
    /// Whether the bonus is percentage-based (true) or flat additive (false).
    pub percentage: bool,
}

/// Per-character perk point allocations.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserPerkRow {
    /// Character name (primary key).
    pub character_id: String,
    /// Level allocated to perk type 0 (Weight).
    pub perk_type0: i16,
    /// Level allocated to perk type 1 (Health).
    pub perk_type1: i16,
    /// Level allocated to perk type 2 (Mana).
    pub perk_type2: i16,
    /// Level allocated to perk type 3 (Loyalty).
    pub perk_type3: i16,
    /// Level allocated to perk type 4 (Drop).
    pub perk_type4: i16,
    /// Level allocated to perk type 5 (Exp).
    pub perk_type5: i16,
    /// Level allocated to perk type 6 (Coins from Monsters).
    pub perk_type6: i16,
    /// Level allocated to perk type 7 (Coins on NPC).
    pub perk_type7: i16,
    /// Level allocated to perk type 8 (Upgrade Chance).
    pub perk_type8: i16,
    /// Level allocated to perk type 9 (Damage to Monsters).
    pub perk_type9: i16,
    /// Level allocated to perk type 10 (Damage to Player).
    pub perk_type10: i16,
    /// Level allocated to perk type 11 (Defence).
    pub perk_type11: i16,
    /// Level allocated to perk type 12 (Attack).
    pub perk_type12: i16,
    /// Unspent perk points.
    pub rem_perk: i16,
}

impl UserPerkRow {
    /// Convert the 13 perk columns into an array.
    pub fn to_array(&self) -> [i16; PERK_COUNT] {
        [
            self.perk_type0,
            self.perk_type1,
            self.perk_type2,
            self.perk_type3,
            self.perk_type4,
            self.perk_type5,
            self.perk_type6,
            self.perk_type7,
            self.perk_type8,
            self.perk_type9,
            self.perk_type10,
            self.perk_type11,
            self.perk_type12,
        ]
    }
}
