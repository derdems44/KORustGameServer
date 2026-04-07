-- Cinderella War (Fun Class) per-class stat/skill presets per tier.
-- C++ Reference: CINDWAR_STAT table, 16 rows (4 tiers x 4 classes).
-- settingid 4 has no stat entries in MSSQL.

CREATE TABLE IF NOT EXISTS cindwar_stat (
    id              INTEGER     NOT NULL PRIMARY KEY,
    setting_id      SMALLINT    NOT NULL,
    class           SMALLINT    NOT NULL,
    skill_freepoint SMALLINT    NOT NULL DEFAULT 0,
    skill_page1     SMALLINT    NOT NULL DEFAULT 0,
    skill_page2     SMALLINT    NOT NULL DEFAULT 0,
    skill_page3     SMALLINT    NOT NULL DEFAULT 0,
    skill_page4     SMALLINT    NOT NULL DEFAULT 0,
    stat_str        SMALLINT    NOT NULL DEFAULT 0,
    stat_sta        SMALLINT    NOT NULL DEFAULT 0,
    stat_dex        SMALLINT    NOT NULL DEFAULT 0,
    stat_int        SMALLINT    NOT NULL DEFAULT 0,
    stat_cha        SMALLINT    NOT NULL DEFAULT 0,
    stat_freepoint  SMALLINT    NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_cindwar_stat_setting_class ON cindwar_stat (setting_id, class);

INSERT INTO cindwar_stat (id, setting_id, class, skill_freepoint, skill_page1, skill_page2, skill_page3, skill_page4, stat_str, stat_sta, stat_dex, stat_int, stat_cha, stat_freepoint) VALUES
(1,  0, 1, 0, 0,  47, 60, 5,  252, 90,  60,  50,  50,  0),
(2,  0, 2, 0, 0,  60, 47, 5,  60,  90,  252, 50,  50,  0),
(3,  0, 3, 0, 60, 47, 0,  5,  50,  60,  60,  112, 220, 0),
(4,  0, 4, 0, 54, 0,  57, 1,  102, 166, 60,  124, 50,  0),
(5,  1, 1, 0, 45, 31, 0,  0,  208, 70,  60,  50,  50,  0),
(6,  1, 2, 0, 0,  45, 31, 0,  60,  70,  208, 50,  50,  0),
(7,  1, 3, 0, 45, 31, 0,  0,  50,  60,  60,  70,  198, 0),
(8,  1, 4, 0, 31, 0,  45, 0,  71,  187, 60,  70,  50,  0),
(9,  2, 1, 0, 57, 43, 0,  0,  232, 82,  60,  50,  50,  0),
(10, 2, 2, 0, 0,  55, 45, 0,  60,  82,  232, 50,  50,  0),
(11, 2, 3, 0, 55, 45, 0,  0,  50,  60,  60,  112, 192, 0),
(12, 2, 4, 0, 43, 0,  57, 0,  99,  141, 60,  124, 50,  0),
(13, 3, 1, 0, 0,  50, 83, 15, 255, 177, 60,  50,  50,  0),
(14, 3, 2, 0, 0,  80, 45, 23, 60,  177, 255, 50,  50,  0),
(15, 3, 3, 0, 80, 45, 0,  23, 50,  115, 60,  112, 255, 0),
(16, 3, 4, 0, 60, 0,  80, 8,  103, 255, 60,  124, 50,  0)
ON CONFLICT DO NOTHING;
