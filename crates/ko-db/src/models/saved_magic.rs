//! Saved magic (buff persistence) model — maps to `user_saved_magic` table.
//! Persists active buffs across logout and zone changes.

/// A single saved buff entry (one slot in the 10-slot persistence system).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SavedMagicRow {
    /// Character name (PK part 1).
    pub character_id: String,
    /// Slot index 0-9 (PK part 2).
    pub slot: i16,
    /// Skill ID (magic_num) of the saved buff. 0 = empty slot.
    pub skill_id: i32,
    /// Remaining duration in seconds when saved. 0 = expired/empty.
    pub remaining_duration: i32,
}
