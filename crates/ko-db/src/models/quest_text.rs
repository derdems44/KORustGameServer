//! Quest text reference table models — NPC dialog menus and talk text.
//! - `GameServerDlg.cpp` — `m_QuestMenuArray`, `m_QuestTalkArray`
//! - `shared/database/QuestMenuSet.h`, `shared/database/QuestTalkSet.h`
//! These tables are bulk-loaded at startup and cached in WorldState for
//! fast dialog text lookup by quest Lua scripts.

/// Quest menu option from the `quest_menu` table.
/// Defines NPC dialog menu choices shown to the player.
/// MSSQL source: `QUEST_MENU_US` (3,006 rows).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestMenuRow {
    /// Menu option ID (primary key).
    pub i_num: i32,
    /// Menu option text shown to the player (max 100 chars).
    pub str_menu: String,
}

/// Quest talk text from the `quest_talk` table.
/// Defines NPC dialog body text shown to the player.
/// MSSQL source: `QUEST_TALK_US` (12,060 rows).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestTalkRow {
    /// Talk text ID (primary key).
    pub i_num: i32,
    /// Dialog text shown to the player (max 1000 chars).
    /// May contain `<selfname>` placeholder replaced at runtime.
    pub str_talk: String,
}

/// Quest skill closed check entry from `quest_skills_closed_check` table.
/// Defines prerequisites for skill quest completion.
/// MSSQL source: `QUEST_SKILLS_CLOSED_CHECK` (25 rows).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestSkillsClosedCheckRow {
    /// Unique index (primary key).
    pub n_index: i32,
    /// Event data index referencing the skill quest.
    pub s_event_data_index: i16,
    /// Nation filter: 1=Karus, 3=El Morad (NULL=any).
    pub n_nation: Option<i16>,
}

/// Quest skill open setup entry from `quest_skills_open_set_up` table.
/// Defines conditions for opening skill quests.
/// MSSQL source: `QUEST_SKILLS_OPEN_SET_UP` (20 rows).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestSkillsOpenSetUpRow {
    /// Unique index (primary key).
    pub n_index: i32,
    /// Event data index referencing the skill quest.
    pub n_event_data_index: i16,
}

/// Per-character skill quest progress from `quest_skills_closed_data` table.
/// Stores binary quest skill progress blob per character.
/// MSSQL source: `QUEST_SKILLS_CLOSED_DATA` (8,343 rows — per-user).
/// Schema-only migration; data is loaded per-character at login.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestSkillsClosedDataRow {
    /// Character name (foreign key to userdata).
    pub str_user_id: String,
    /// Binary quest skill progress data (3888 bytes in MSSQL).
    pub str_quest_skill: Option<Vec<u8>>,
    /// Count of completed skill quests.
    pub str_quest_skill_count: Option<i16>,
    /// Check flag.
    pub str_check: Option<i16>,
}
