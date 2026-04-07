-- Dungeon Defence (Full Moon Rift) monster spawn list.
-- Migrated from MSSQL DF_MONSTER_LIST (35 rows).
-- C++ Reference: _DUNGEON_DEFENCE_MONSTER_LIST struct, m_DungeonDefenceMonsterListArray

CREATE TABLE IF NOT EXISTS df_monster_list (
    id              INTEGER     NOT NULL PRIMARY KEY,
    difficulty      SMALLINT,
    monster_id      SMALLINT    NOT NULL,
    is_monster      BOOLEAN     NOT NULL DEFAULT FALSE,
    pos_x           SMALLINT    NOT NULL DEFAULT 60,
    pos_z           SMALLINT    NOT NULL DEFAULT 42,
    s_count         SMALLINT    DEFAULT 1,
    s_direction     SMALLINT    NOT NULL DEFAULT 0,
    s_radius_range  SMALLINT    DEFAULT 0
);

INSERT INTO df_monster_list (id, difficulty, monster_id, is_monster, pos_x, pos_z, s_count, s_direction, s_radius_range) VALUES
(1, 1, 9927, FALSE, 60, 42, 2, 0, 10),
(2, 1, 9928, FALSE, 60, 42, 1, 0, 0),
(3, 1, 9929, FALSE, 60, 42, 2, 0, 10),
(4, 1, 9930, FALSE, 60, 42, 2, 0, 10),
(5, 1, 9931, FALSE, 60, 42, 1, 0, 0),
(6, 2, 9947, FALSE, 60, 42, 4, 0, 10),
(7, 2, 9948, FALSE, 60, 42, 2, 0, 10),
(8, 2, 9949, FALSE, 60, 42, 3, 0, 10),
(9, 2, 9950, FALSE, 60, 42, 3, 0, 10),
(10, 2, 9951, FALSE, 60, 42, 1, 0, 0),
(11, 2, 9932, FALSE, 60, 42, 3, 0, 10),
(12, 2, 9933, FALSE, 60, 42, 2, 0, 10),
(13, 2, 9932, FALSE, 60, 42, 3, 0, 10),
(14, 2, 9933, FALSE, 60, 42, 3, 0, 10),
(15, 2, 9934, FALSE, 60, 42, 3, 0, 10),
(16, 2, 9935, FALSE, 60, 42, 2, 0, 10),
(17, 2, 9936, FALSE, 60, 42, 1, 0, 0),
(18, 3, 9955, FALSE, 60, 42, 4, 0, 10),
(19, 3, 9956, FALSE, 60, 42, 2, 0, 10),
(20, 3, 9957, FALSE, 60, 42, 3, 0, 10),
(21, 3, 9958, FALSE, 60, 42, 3, 0, 10),
(22, 3, 9959, FALSE, 60, 42, 1, 0, 0),
(23, 3, 9932, FALSE, 60, 42, 3, 0, 10),
(24, 3, 9933, FALSE, 60, 42, 2, 0, 10),
(25, 3, 9932, FALSE, 60, 42, 4, 0, 10),
(26, 3, 9933, FALSE, 60, 42, 3, 0, 10),
(27, 3, 9934, FALSE, 60, 42, 3, 0, 10),
(28, 3, 9935, FALSE, 60, 42, 2, 0, 10),
(29, 3, 9936, FALSE, 60, 42, 1, 0, 0),
(30, 3, 9937, FALSE, 60, 42, 4, 0, 10),
(31, 3, 9938, FALSE, 60, 42, 2, 0, 10),
(32, 3, 9939, FALSE, 60, 42, 3, 0, 10),
(33, 3, 9940, FALSE, 60, 42, 3, 0, 10),
(34, 3, 9941, FALSE, 60, 42, 2, 0, 10),
(35, 3, 9942, FALSE, 60, 42, 1, 0, 0)
ON CONFLICT DO NOTHING;
