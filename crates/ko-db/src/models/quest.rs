//! Quest-related reference table models — maps to PostgreSQL tables.
//! - `GameDefine.h:3051` — `_QUEST_HELPER`
//! - `GameDefine.h:3154` — `_QUEST_MONSTER`
//! - `GameDefine.h:103`  — `_USER_QUEST_INFO`
//! - `shared/database/QuestHelperSet.h`
//! - `shared/database/QuestMonsterSet.h`
//! These tables are bulk-loaded at startup and cached in WorldState.

/// Quest helper entry from the `quest_helper` table.
/// Defines quest prerequisites, NPC associations, and event triggers.
/// MSSQL source: `QUEST_HELPER` (7,085 rows).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestHelperRow {
    /// Unique quest helper index (primary key).
    pub n_index: i32,
    /// Message type for the NPC dialog.
    pub b_message_type: i16,
    /// Minimum level required to start this quest.
    pub b_level: i16,
    /// Experience reward / requirement.
    pub n_exp: i32,
    /// Required class (5 = any class).
    ///
    pub b_class: i16,
    /// Required nation (3 = any nation, 1 = Karus, 2 = El Morad).
    ///
    pub b_nation: i16,
    /// Quest type (1 = normal, 4 = special, etc.).
    pub b_quest_type: i16,
    /// Zone where the quest takes place.
    pub b_zone: i16,
    /// NPC ID that gives this quest.
    pub s_npc_id: i16,
    /// Event data index — the quest ID used in player quest maps.
    ///
    pub s_event_data_index: i16,
    /// Required quest state to show this quest step.
    ///
    pub b_event_status: i16,
    /// Lua event trigger index (for quest accept/start).
    pub n_event_trigger_index: i32,
    /// Lua event complete index (for quest turn-in).
    pub n_event_complete_index: i32,
    /// Exchange index for quest item trades.
    pub n_exchange_index: i32,
    /// Lua event talk index.
    pub n_event_talk_index: i32,
    /// Lua script filename for this quest.
    pub str_lua_filename: String,
    /// Quest menu ID for the NPC dialog.
    pub s_quest_menu: i32,
    /// NPC main action ID.
    pub s_npc_main: i32,
    /// Whether this quest is solo-only.
    pub s_quest_solo: i16,
}

/// Quest monster entry from the `quest_monster` table.
/// Defines which monsters must be killed for a quest and their required counts.
/// MSSQL source: `QUEST_MONSTER` (606 rows).
/// Structure: 4 groups, each with 4 monster IDs and 1 required count.
/// C++ constants: `QUEST_MOB_GROUPS = 4`, `QUEST_MOBS_PER_GROUP = 4`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QuestMonsterRow {
    /// Quest number (sEventDataIndex — primary key).
    pub s_quest_num: i16,
    // ── Group 1 ──
    pub s_num1a: i16,
    pub s_num1b: i16,
    pub s_num1c: i16,
    pub s_num1d: i16,
    pub s_count1: i16,
    // ── Group 2 ──
    pub s_num2a: i16,
    pub s_num2b: i16,
    pub s_num2c: i16,
    pub s_num2d: i16,
    pub s_count2: i16,
    // ── Group 3 ──
    pub s_num3a: i16,
    pub s_num3b: i16,
    pub s_num3c: i16,
    pub s_num3d: i16,
    pub s_count3: i16,
    // ── Group 4 ──
    pub s_num4a: i16,
    pub s_num4b: i16,
    pub s_num4c: i16,
    pub s_num4d: i16,
    pub s_count4: i16,
}

/// Per-player quest progress row from the `user_quest` table.
/// Storage: quest_id (u16) + state (u8) + 4 kill counts (u8 each) = 7 bytes per quest in C++.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserQuestRow {
    /// Character name (foreign key to userdata).
    pub str_user_id: String,
    /// Quest ID (sEventDataIndex).
    pub quest_id: i16,
    /// Quest state: 0=not started, 1=ongoing, 2=completed, 3=ready to complete, 4=removed.
    pub quest_state: i16,
    /// Kill count for monster group 1.
    pub kill_count1: i16,
    /// Kill count for monster group 2.
    pub kill_count2: i16,
    /// Kill count for monster group 3.
    pub kill_count3: i16,
    /// Kill count for monster group 4.
    pub kill_count4: i16,
}
