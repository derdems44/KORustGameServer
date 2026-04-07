-- Per-character skill quest progress storage
-- MSSQL source: QUEST_SKILLS_CLOSED_DATA (per-user, schema only)
-- C++ Reference: CUser quest skill blob

CREATE TABLE IF NOT EXISTS quest_skills_closed_data (
    str_user_id             VARCHAR(21)  NOT NULL PRIMARY KEY,
    str_quest_skill         BYTEA,
    str_quest_skill_count   SMALLINT,
    str_check               SMALLINT
);

CREATE INDEX IF NOT EXISTS idx_quest_skills_closed_data_user
    ON quest_skills_closed_data (str_user_id);
