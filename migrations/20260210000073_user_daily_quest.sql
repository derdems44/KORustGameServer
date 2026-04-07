-- User Daily Quest progress table
-- Migrated from MSSQL USER_DAILY_QUEST
-- Tracks per-character progress on daily quests.
-- C++ Reference: _DAILY_USERQUEST struct, DailyQuestMap per user

CREATE TABLE IF NOT EXISTS user_daily_quest (
    character_id VARCHAR(20) NOT NULL,
    quest_id SMALLINT NOT NULL,
    kill_count INTEGER NOT NULL DEFAULT 0,
    status SMALLINT NOT NULL DEFAULT 2,
    replay_time INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT pk_user_daily_quest PRIMARY KEY (character_id, quest_id)
);

CREATE INDEX IF NOT EXISTS idx_user_daily_quest_char ON user_daily_quest(character_id);
