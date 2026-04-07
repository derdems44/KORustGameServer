//! Skill shortcut bar model — maps to `user_skill_shortcuts` table.
//! Stores the hotbar layout as a binary blob of uint32 skill IDs.

/// A single skill shortcut row — one per character.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SkillShortcutRow {
    /// Character name (primary key).
    pub character_id: String,
    /// Number of skill slots saved.
    pub count: i16,
    /// Raw binary skill data — each slot is 4 bytes (little-endian uint32).
    pub skill_data: Vec<u8>,
}
