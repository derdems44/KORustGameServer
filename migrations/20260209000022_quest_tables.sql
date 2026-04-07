-- Quest-related reference tables migrated from MSSQL
-- Tables: quest_helper, quest_monster, user_quest
-- Data counts: quest_helper(7,085), quest_monster(606)
-- C++ Reference: GameDefine.h _QUEST_HELPER, _QUEST_MONSTER, _USER_QUEST_INFO

-----------------------------------------------
-- quest_helper: Quest definitions and NPC associations
-- MSSQL source: QUEST_HELPER (7,085 rows)
-- C++ Reference: GameDefine.h:3051 _QUEST_HELPER
-----------------------------------------------
CREATE TABLE IF NOT EXISTS quest_helper (
    n_index                 INTEGER   NOT NULL PRIMARY KEY,
    b_message_type          SMALLINT  NOT NULL DEFAULT 0,
    b_level                 SMALLINT  NOT NULL DEFAULT 0,
    n_exp                   INTEGER   NOT NULL DEFAULT 0,
    b_class                 SMALLINT  NOT NULL DEFAULT 5,
    b_nation                SMALLINT  NOT NULL DEFAULT 3,
    b_quest_type            SMALLINT  NOT NULL DEFAULT 0,
    b_zone                  SMALLINT  NOT NULL DEFAULT 0,
    s_npc_id                SMALLINT  NOT NULL DEFAULT 0,
    s_event_data_index      SMALLINT  NOT NULL DEFAULT 0,
    b_event_status          SMALLINT  NOT NULL DEFAULT 0,
    n_event_trigger_index   INTEGER   NOT NULL DEFAULT 0,
    n_event_complete_index  INTEGER   NOT NULL DEFAULT 0,
    n_exchange_index        INTEGER   NOT NULL DEFAULT 0,
    n_event_talk_index      INTEGER   NOT NULL DEFAULT 0,
    str_lua_filename        VARCHAR(40) NOT NULL DEFAULT '',
    s_quest_menu            INTEGER   NOT NULL DEFAULT 0,
    s_npc_main              INTEGER   NOT NULL DEFAULT 0,
    s_quest_solo            SMALLINT  NOT NULL DEFAULT 0
);
CREATE INDEX idx_quest_helper_npc ON quest_helper (s_npc_id);
CREATE INDEX idx_quest_helper_event ON quest_helper (s_event_data_index);

-----------------------------------------------
-- quest_monster: Monster kill requirements per quest
-- MSSQL source: QUEST_MONSTER (606 rows)
-- C++ Reference: GameDefine.h:3154 _QUEST_MONSTER
-- 4 groups x (4 monster IDs + 1 required count)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS quest_monster (
    s_quest_num   SMALLINT  NOT NULL PRIMARY KEY,
    s_num1a       SMALLINT  NOT NULL DEFAULT 0,
    s_num1b       SMALLINT  NOT NULL DEFAULT 0,
    s_num1c       SMALLINT  NOT NULL DEFAULT 0,
    s_num1d       SMALLINT  NOT NULL DEFAULT 0,
    s_count1      SMALLINT  NOT NULL DEFAULT 0,
    s_num2a       SMALLINT  NOT NULL DEFAULT 0,
    s_num2b       SMALLINT  NOT NULL DEFAULT 0,
    s_num2c       SMALLINT  NOT NULL DEFAULT 0,
    s_num2d       SMALLINT  NOT NULL DEFAULT 0,
    s_count2      SMALLINT  NOT NULL DEFAULT 0,
    s_num3a       SMALLINT  NOT NULL DEFAULT 0,
    s_num3b       SMALLINT  NOT NULL DEFAULT 0,
    s_num3c       SMALLINT  NOT NULL DEFAULT 0,
    s_num3d       SMALLINT  NOT NULL DEFAULT 0,
    s_count3      SMALLINT  NOT NULL DEFAULT 0,
    s_num4a       SMALLINT  NOT NULL DEFAULT 0,
    s_num4b       SMALLINT  NOT NULL DEFAULT 0,
    s_num4c       SMALLINT  NOT NULL DEFAULT 0,
    s_num4d       SMALLINT  NOT NULL DEFAULT 0,
    s_count4      SMALLINT  NOT NULL DEFAULT 0
);

-----------------------------------------------
-- user_quest: Per-player quest progress
-- C++ Reference: GameDefine.h:103 _USER_QUEST_INFO
-- Originally stored as binary blob in C++, normalized here
-----------------------------------------------
CREATE TABLE IF NOT EXISTS user_quest (
    str_user_id   VARCHAR(21) NOT NULL,
    quest_id      SMALLINT    NOT NULL,
    quest_state   SMALLINT    NOT NULL DEFAULT 0,
    kill_count1   SMALLINT    NOT NULL DEFAULT 0,
    kill_count2   SMALLINT    NOT NULL DEFAULT 0,
    kill_count3   SMALLINT    NOT NULL DEFAULT 0,
    kill_count4   SMALLINT    NOT NULL DEFAULT 0,
    PRIMARY KEY (str_user_id, quest_id)
);
CREATE INDEX idx_user_quest_user ON user_quest (str_user_id);
