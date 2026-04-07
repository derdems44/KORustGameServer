-- Achievement system reference tables.
-- C++ Reference: GameDefine.h — _ACHIEVE_MAIN, _ACHIEVE_WAR, _ACHIEVE_NORMAL, _ACHIEVE_MONSTER, _ACHIEVE_COM, _ACHIEVE_TITLE
-- MSSQL source: ACHIEVE_MAIN (456 rows), ACHIEVE_WAR (92), ACHIEVE_NORMAL (46),
--               ACHIEVE_MON (248), ACHIEVE_COM (71), ACHIEVE_TITLE (136)
-- Data is bulk-loaded via Python (same pattern as item table).

-- ─── achieve_main ────────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_MAIN (GameDefine.h:2357)
-- Master achievement definition table.
CREATE TABLE IF NOT EXISTS achieve_main (
    s_index         INT         NOT NULL PRIMARY KEY,
    type            SMALLINT    NOT NULL DEFAULT 0,  -- Maps to UserAchieveMainTypes (1=War,2=Monster,3=Com,4=Normal)
    title_id        SMALLINT    NOT NULL DEFAULT 0,
    point           SMALLINT    NOT NULL DEFAULT 0,  -- Medal points awarded
    item_num        INT         NOT NULL DEFAULT 0,  -- Reward item ID
    count           INT         NOT NULL DEFAULT 0,  -- Reward item count
    zone_id         SMALLINT    NOT NULL DEFAULT 0,  -- Required zone (0=any)
    unknown2        SMALLINT    NOT NULL DEFAULT 0,
    achieve_type    SMALLINT    NOT NULL DEFAULT 0,  -- 0=Normal,1=Quest,2=War,3=Adventure,4=Challenge
    req_time        SMALLINT    NOT NULL DEFAULT 0,  -- Time limit in seconds (0=no limit)
    byte1           SMALLINT    NOT NULL DEFAULT 0,
    byte2           SMALLINT    NOT NULL DEFAULT 0
);

-- ─── achieve_war ─────────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_WAR (GameDefine.h:2407)
-- War-type achievement sub-table.
CREATE TABLE IF NOT EXISTS achieve_war (
    s_index     INT         NOT NULL PRIMARY KEY,
    type        SMALLINT    NOT NULL DEFAULT 0,  -- UserAchieveWarTypes enum
    s_count     INT         NOT NULL DEFAULT 0   -- Required count to complete
);

-- ─── achieve_normal ──────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_NORMAL (GameDefine.h:2414)
-- Normal-type achievement sub-table.
CREATE TABLE IF NOT EXISTS achieve_normal (
    s_index     INT         NOT NULL PRIMARY KEY,
    type        SMALLINT    NOT NULL DEFAULT 0,  -- UserAchieveNormalTypes enum
    count       INT         NOT NULL DEFAULT 0   -- Required count to complete
);

-- ─── achieve_monster ─────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_MONSTER (GameDefine.h:2398)
-- Monster-kill achievement sub-table. 2 groups x 4 monsters each.
CREATE TABLE IF NOT EXISTS achieve_monster (
    s_index     INT         NOT NULL PRIMARY KEY,
    type        SMALLINT    NOT NULL DEFAULT 0,
    byte        SMALLINT    NOT NULL DEFAULT 0,
    -- Group 1: 4 monster IDs + required kill count
    monster1_1  INT         NOT NULL DEFAULT 0,
    monster1_2  INT         NOT NULL DEFAULT 0,
    monster1_3  INT         NOT NULL DEFAULT 0,
    monster1_4  INT         NOT NULL DEFAULT 0,
    mon_count1  INT         NOT NULL DEFAULT 0,
    -- Group 2: 4 monster IDs + required kill count
    monster2_1  INT         NOT NULL DEFAULT 0,
    monster2_2  INT         NOT NULL DEFAULT 0,
    monster2_3  INT         NOT NULL DEFAULT 0,
    monster2_4  INT         NOT NULL DEFAULT 0,
    mon_count2  INT         NOT NULL DEFAULT 0
);

-- ─── achieve_com ─────────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_COM (GameDefine.h:2421)
-- Composite (requirement-based) achievement sub-table.
CREATE TABLE IF NOT EXISTS achieve_com (
    s_index     INT         NOT NULL PRIMARY KEY,
    type        SMALLINT    NOT NULL DEFAULT 0,  -- UserAchieveComTypes (1=RequireQuest,2=RequireAchieve)
    req1        INT         NOT NULL DEFAULT 0,  -- Required quest/achieve ID #1
    req2        INT         NOT NULL DEFAULT 0   -- Required quest/achieve ID #2 (0=none)
);

-- ─── achieve_title ───────────────────────────────────────────────────────
-- C++ Reference: _ACHIEVE_TITLE (GameDefine.h:2291)
-- Skill title bonuses for equipped achievement titles.
CREATE TABLE IF NOT EXISTS achieve_title (
    s_index         INT         NOT NULL PRIMARY KEY,
    str             SMALLINT    NOT NULL DEFAULT 0,
    hp              SMALLINT    NOT NULL DEFAULT 0,
    dex             SMALLINT    NOT NULL DEFAULT 0,
    "int"           SMALLINT    NOT NULL DEFAULT 0,
    mp              SMALLINT    NOT NULL DEFAULT 0,
    attack          SMALLINT    NOT NULL DEFAULT 0,
    defence         SMALLINT    NOT NULL DEFAULT 0,
    s_loyalty_bonus SMALLINT    NOT NULL DEFAULT 0,
    s_exp_bonus     SMALLINT    NOT NULL DEFAULT 0,
    s_short_sword_ac SMALLINT   NOT NULL DEFAULT 0,
    s_jamadar_ac    SMALLINT    NOT NULL DEFAULT 0,
    s_sword_ac      SMALLINT    NOT NULL DEFAULT 0,
    s_blow_ac       SMALLINT    NOT NULL DEFAULT 0,
    s_axe_ac        SMALLINT    NOT NULL DEFAULT 0,
    s_spear_ac      SMALLINT    NOT NULL DEFAULT 0,
    s_arrow_ac      SMALLINT    NOT NULL DEFAULT 0,
    s_fire_bonus    SMALLINT    NOT NULL DEFAULT 0,
    s_ice_bonus     SMALLINT    NOT NULL DEFAULT 0,
    s_light_bonus   SMALLINT    NOT NULL DEFAULT 0,
    s_fire_resist   SMALLINT    NOT NULL DEFAULT 0,
    s_ice_resist    SMALLINT    NOT NULL DEFAULT 0,
    s_light_resist  SMALLINT    NOT NULL DEFAULT 0,
    s_magic_resist  SMALLINT    NOT NULL DEFAULT 0,
    s_curse_resist  SMALLINT    NOT NULL DEFAULT 0,
    s_poison_resist SMALLINT    NOT NULL DEFAULT 0
);

-- ─── user_achieve ────────────────────────────────────────────────────────
-- Per-player achievement progress (replaces MSSQL's binary blob approach).
-- C++ Reference: USER_ACHIEVE_DATA.strAchieve binary blob → normalized rows.
CREATE TABLE IF NOT EXISTS user_achieve (
    str_user_id     VARCHAR(21) NOT NULL,
    achieve_id      INT         NOT NULL,
    status          SMALLINT    NOT NULL DEFAULT 1,  -- UserAchieveStatus: 0=ChallengeIncomplete,1=Incomplete,4=Finished,5=Completed
    count1          INT         NOT NULL DEFAULT 0,  -- Progress counter (group 1)
    count2          INT         NOT NULL DEFAULT 0,  -- Progress counter (group 2)
    PRIMARY KEY (str_user_id, achieve_id)
);

-- ─── user_achieve_summary ────────────────────────────────────────────────
-- Per-player achievement summary stats.
-- C++ Reference: _ACHIEVE_INFO struct fields serialized in USER_ACHIEVE_DATA.strAchieve bytes 0-35.
CREATE TABLE IF NOT EXISTS user_achieve_summary (
    str_user_id         VARCHAR(21) NOT NULL PRIMARY KEY,
    play_time           INT         NOT NULL DEFAULT 0,   -- Total play time in seconds
    monster_defeat_count INT        NOT NULL DEFAULT 0,
    user_defeat_count   INT         NOT NULL DEFAULT 0,
    user_death_count    INT         NOT NULL DEFAULT 0,
    total_medal         INT         NOT NULL DEFAULT 0,
    recent_achieve_1    SMALLINT    NOT NULL DEFAULT 0,
    recent_achieve_2    SMALLINT    NOT NULL DEFAULT 0,
    recent_achieve_3    SMALLINT    NOT NULL DEFAULT 0,
    cover_id            SMALLINT    NOT NULL DEFAULT 0,   -- Equipped cover title achieve ID
    skill_id            SMALLINT    NOT NULL DEFAULT 0    -- Equipped skill title achieve ID
);
