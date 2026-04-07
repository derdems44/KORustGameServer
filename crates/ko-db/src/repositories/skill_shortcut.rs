//! Skill shortcut repository — user_skill_shortcuts table access.
//!
//! C++ Reference: `CDBAgent::LoadSkillShortcut` and `CDBAgent::SaveSkillShortcut`
//! in `DBAgent.cpp:1197-1238`.

use sqlx::PgPool;

use crate::models::SkillShortcutRow;

/// Maximum number of skill shortcut slots (320 bytes / 4 bytes per slot).
pub const MAX_SKILL_SHORTCUT_SLOTS: usize = 80;

/// Maximum byte size of skill data buffer (matches C++ `char m_strSkillData[320]`).
pub const MAX_SKILL_DATA_BYTES: usize = MAX_SKILL_SHORTCUT_SLOTS * 4;

/// Repository for skill shortcut database operations.
pub struct SkillShortcutRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SkillShortcutRepository<'a> {
    /// Create a new skill shortcut repository.
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Load skill shortcut data for a character.
    ///
    /// Returns `None` if no data has been saved yet.
    ///
    /// C++ Reference: `CDBAgent::LoadSkillShortcut`
    pub async fn load(&self, character_id: &str) -> Result<Option<SkillShortcutRow>, sqlx::Error> {
        sqlx::query_as::<_, SkillShortcutRow>(
            "SELECT character_id, count, skill_data FROM user_skill_shortcuts WHERE character_id = $1",
        )
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
    }

    /// Save (upsert) skill shortcut data for a character.
    ///
    /// C++ Reference: `CDBAgent::SaveSkillShortcut` — calls `SKILLSHORTCUT_SAVE` stored proc.
    pub async fn save(
        &self,
        character_id: &str,
        count: i16,
        skill_data: &[u8],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_skill_shortcuts (character_id, count, skill_data) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (character_id) DO UPDATE SET count = $2, skill_data = $3",
        )
        .bind(character_id)
        .bind(count)
        .bind(skill_data)
        .execute(self.pool)
        .await?;

        Ok(())
    }
}
