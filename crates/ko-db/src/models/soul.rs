//! Soul system models — maps to `user_soul_data` table.
//!
//! v2525-specific: WIZ_SOUL (0xC5) panel data. 8 categories × 3 values + 5 slots.

/// Number of soul categories (0-7).
pub const SOUL_CATEGORY_COUNT: usize = 8;

/// Number of soul slots (0-4).
pub const SOUL_SLOT_COUNT: usize = 5;

/// A row from the `user_soul_data` table — per-character soul state.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserSoulDataRow {
    /// Character name (primary key).
    pub character_id: String,

    // ── Categories (8 × 3 values) ──────────────────────────────
    pub cat0_v0: i16,
    pub cat0_v1: i16,
    pub cat0_v2: i16,
    pub cat1_v0: i16,
    pub cat1_v1: i16,
    pub cat1_v2: i16,
    pub cat2_v0: i16,
    pub cat2_v1: i16,
    pub cat2_v2: i16,
    pub cat3_v0: i16,
    pub cat3_v1: i16,
    pub cat3_v2: i16,
    pub cat4_v0: i16,
    pub cat4_v1: i16,
    pub cat4_v2: i16,
    pub cat5_v0: i16,
    pub cat5_v1: i16,
    pub cat5_v2: i16,
    pub cat6_v0: i16,
    pub cat6_v1: i16,
    pub cat6_v2: i16,
    pub cat7_v0: i16,
    pub cat7_v1: i16,
    pub cat7_v2: i16,

    // ── Slots (5 values) ───────────────────────────────────────
    pub slot0: i16,
    pub slot1: i16,
    pub slot2: i16,
    pub slot3: i16,
    pub slot4: i16,
}

impl UserSoulDataRow {
    /// Extract category values as an array of `[cat_id, v0, v1, v2]` tuples.
    pub fn categories(&self) -> [[i16; 4]; SOUL_CATEGORY_COUNT] {
        [
            [0, self.cat0_v0, self.cat0_v1, self.cat0_v2],
            [1, self.cat1_v0, self.cat1_v1, self.cat1_v2],
            [2, self.cat2_v0, self.cat2_v1, self.cat2_v2],
            [3, self.cat3_v0, self.cat3_v1, self.cat3_v2],
            [4, self.cat4_v0, self.cat4_v1, self.cat4_v2],
            [5, self.cat5_v0, self.cat5_v1, self.cat5_v2],
            [6, self.cat6_v0, self.cat6_v1, self.cat6_v2],
            [7, self.cat7_v0, self.cat7_v1, self.cat7_v2],
        ]
    }

    /// Extract slot values as `[slot_id, value]` pairs.
    pub fn slots(&self) -> [[i16; 2]; SOUL_SLOT_COUNT] {
        [
            [0, self.slot0],
            [1, self.slot1],
            [2, self.slot2],
            [3, self.slot3],
            [4, self.slot4],
        ]
    }

    /// Returns true if all values are zero (no soul data).
    pub fn is_empty(&self) -> bool {
        self.categories()
            .iter()
            .all(|c| c[1] == 0 && c[2] == 0 && c[3] == 0)
            && self.slots().iter().all(|s| s[1] == 0)
    }
}
