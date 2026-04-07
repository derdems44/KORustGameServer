//! Quest text repository — database access for quest menu and talk tables.
//! - `GameServerDlg.cpp` — loads `QUEST_MENU_US`, `QUEST_TALK_US` at startup
//! - `CQuestMenuSet`, `CQuestTalkSet` — recordset loaders

use crate::models::quest_text::{
    QuestMenuRow, QuestSkillsClosedCheckRow, QuestSkillsClosedDataRow, QuestSkillsOpenSetUpRow,
    QuestTalkRow,
};
use crate::DbPool;

/// Repository for quest text DB operations.
pub struct QuestTextRepository<'a> {
    pool: &'a DbPool,
}

impl<'a> QuestTextRepository<'a> {
    /// Create a new quest text repository.
    pub fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Load all quest menu options.
    ///
    pub async fn load_quest_menus(&self) -> Result<Vec<QuestMenuRow>, sqlx::Error> {
        sqlx::query_as::<_, QuestMenuRow>("SELECT * FROM quest_menu ORDER BY i_num")
            .fetch_all(self.pool)
            .await
    }

    /// Load all quest talk text entries.
    ///
    pub async fn load_quest_talks(&self) -> Result<Vec<QuestTalkRow>, sqlx::Error> {
        sqlx::query_as::<_, QuestTalkRow>("SELECT * FROM quest_talk ORDER BY i_num")
            .fetch_all(self.pool)
            .await
    }

    /// Load all quest skill closed check entries.
    pub async fn load_quest_skills_closed_check(
        &self,
    ) -> Result<Vec<QuestSkillsClosedCheckRow>, sqlx::Error> {
        sqlx::query_as::<_, QuestSkillsClosedCheckRow>(
            "SELECT * FROM quest_skills_closed_check ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load all quest skill open setup entries.
    pub async fn load_quest_skills_open_set_up(
        &self,
    ) -> Result<Vec<QuestSkillsOpenSetUpRow>, sqlx::Error> {
        sqlx::query_as::<_, QuestSkillsOpenSetUpRow>(
            "SELECT * FROM quest_skills_open_set_up ORDER BY n_index",
        )
        .fetch_all(self.pool)
        .await
    }

    /// Load quest skill closed data for a specific character.
    pub async fn load_quest_skills_closed_data(
        &self,
        char_name: &str,
    ) -> Result<Option<QuestSkillsClosedDataRow>, sqlx::Error> {
        sqlx::query_as::<_, QuestSkillsClosedDataRow>(
            "SELECT * FROM quest_skills_closed_data WHERE str_user_id = $1",
        )
        .bind(char_name)
        .fetch_optional(self.pool)
        .await
    }

    /// Save or update quest skill closed data for a character.
    pub async fn save_quest_skills_closed_data(
        &self,
        char_name: &str,
        quest_skill: Option<&[u8]>,
        quest_skill_count: Option<i16>,
        check: Option<i16>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO quest_skills_closed_data (str_user_id, str_quest_skill, str_quest_skill_count, str_check)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (str_user_id) DO UPDATE SET
                str_quest_skill = EXCLUDED.str_quest_skill,
                str_quest_skill_count = EXCLUDED.str_quest_skill_count,
                str_check = EXCLUDED.str_check",
        )
        .bind(char_name)
        .bind(quest_skill)
        .bind(quest_skill_count)
        .bind(check)
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
