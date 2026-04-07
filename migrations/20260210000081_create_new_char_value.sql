-- CREATE_NEW_CHAR_VALUE: Starting stats/level/gold per class+job type for new character creation.
-- Source: MSSQL dbo.CREATE_NEW_CHAR_VALUE (25 rows, 5 classes x 5 job types)
-- C++ Reference: CDBAgent::LoadNewCharValue() — updates userdata stats for new chars
CREATE TABLE IF NOT EXISTS create_new_char_value (
    n_index             INTEGER     NOT NULL PRIMARY KEY,
    class_type          SMALLINT    NOT NULL,
    job_type            SMALLINT    NOT NULL DEFAULT 0,
    level               SMALLINT    NOT NULL DEFAULT 1,
    exp                 BIGINT      NOT NULL DEFAULT 0,
    strength            SMALLINT    NOT NULL DEFAULT 0,
    health              SMALLINT    NOT NULL DEFAULT 0,
    dexterity           SMALLINT    NOT NULL DEFAULT 0,
    intelligence        SMALLINT    NOT NULL DEFAULT 0,
    magic_power         SMALLINT    NOT NULL DEFAULT 0,
    free_points         SMALLINT    NOT NULL DEFAULT 0,
    skill_point_free    SMALLINT    NOT NULL DEFAULT 0,
    skill_point_cat1    SMALLINT    NOT NULL DEFAULT 0,
    skill_point_cat2    SMALLINT    NOT NULL DEFAULT 0,
    skill_point_cat3    SMALLINT    NOT NULL DEFAULT 0,
    skill_point_master  SMALLINT    NOT NULL DEFAULT 0,
    gold                INTEGER     NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_create_new_char_value_class ON create_new_char_value (class_type);

-- 25 rows: 5 classes (1,2,3,4,13) x 5 job types (0,1,2,3,4)
INSERT INTO create_new_char_value (n_index, class_type, job_type, level, exp, strength, health, dexterity, intelligence, magic_power, free_points, skill_point_free, skill_point_cat1, skill_point_cat2, skill_point_cat3, skill_point_master, gold) VALUES
-- JobType 0 (base class)
(1, 1, 0, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(2, 2, 0, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(3, 3, 0, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(4, 4, 0, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(5, 13, 0, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
-- JobType 1 (1st class change)
(6, 1, 1, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(7, 2, 1, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(8, 3, 1, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(9, 4, 1, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(10, 13, 1, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
-- JobType 2 (2nd class change)
(11, 1, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(12, 2, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(13, 3, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(14, 4, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
(15, 13, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1000000),
-- JobType 3 (master)
(16, 1, 3, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(17, 2, 3, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(18, 3, 3, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(19, 4, 3, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
(20, 13, 3, 83, 0, 0, 0, 0, 0, 0, 292, 148, 0, 0, 0, 0, 100000000),
-- JobType 4 (grand master)
(21, 1, 4, 80, 0, 0, 0, 0, 0, 0, 277, 142, 0, 0, 0, 0, 100000000),
(22, 2, 4, 80, 0, 0, 0, 0, 0, 0, 277, 142, 0, 0, 0, 0, 100000000),
(23, 3, 4, 80, 0, 0, 0, 0, 0, 0, 277, 142, 0, 0, 0, 0, 100000000),
(24, 4, 4, 80, 0, 0, 0, 0, 0, 0, 277, 142, 0, 0, 0, 0, 100000000),
(25, 13, 4, 80, 0, 0, 0, 0, 0, 0, 277, 142, 0, 0, 0, 0, 1000000000)
ON CONFLICT DO NOTHING;
