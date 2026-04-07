//! Enchant data model — user weapon/armor + item enchant state.

/// Persisted enchant state for a single character.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserEnchant {
    /// Character ID (primary key).
    pub character_id: String,
    /// Highest star tier achieved (weapon/armor).
    pub max_star: i16,
    /// Total enchant count (weapon/armor).
    pub enchant_count: i16,
    /// Slot levels (8 bytes, one per slot).
    pub slot_levels: Vec<u8>,
    /// Slot unlock flags (9 bytes).
    pub slot_unlocked: Vec<u8>,
    /// Item enchant: current category.
    pub item_category: i16,
    /// Item enchant: slot unlock count.
    pub item_slot_unlock: i16,
    /// Item enchant: marker flags (5 bytes).
    pub item_markers: Vec<u8>,
}
