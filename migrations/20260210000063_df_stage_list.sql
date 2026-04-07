-- Dungeon Defence (Full Moon Rift) stage list.
-- Migrated from MSSQL DF_STAGE_LIST (35 rows).
-- C++ Reference: _DUNGEON_DEFENCE_STAGE_LIST struct, m_DungeonDefenceStageListArray

CREATE TABLE IF NOT EXISTS df_stage_list (
    id          INTEGER     NOT NULL PRIMARY KEY,
    difficulty  SMALLINT    NOT NULL,
    difficulty_name VARCHAR(15),
    stage_id    SMALLINT    NOT NULL
);

INSERT INTO df_stage_list (id, difficulty, difficulty_name, stage_id) VALUES
(1, 1, 'Easy', 1),
(3, 1, 'Easy', 2),
(4, 1, 'Easy', 3),
(5, 1, 'Easy', 4),
(6, 1, 'Easy', 5),
(7, 2, 'Normal', 6),
(8, 2, 'Normal', 7),
(9, 2, 'Normal', 8),
(10, 2, 'Normal', 9),
(11, 2, 'Normal', 10),
(12, 2, 'Normal', 11),
(13, 2, 'Normal', 12),
(14, 2, 'Normal', 13),
(15, 2, 'Normal', 14),
(16, 2, 'Normal', 15),
(17, 2, 'Normal', 16),
(18, 2, 'Normal', 17),
(19, 3, 'Hard', 18),
(20, 3, 'Hard', 19),
(21, 3, 'Hard', 20),
(22, 3, 'Hard', 21),
(23, 3, 'Hard', 22),
(24, 3, 'Hard', 23),
(25, 3, 'Hard', 24),
(26, 3, 'Hard', 25),
(27, 3, 'Hard', 26),
(28, 3, 'Hard', 27),
(29, 3, 'Hard', 28),
(30, 3, 'Hard', 29),
(31, 3, 'Hard', 30),
(32, 3, 'Hard', 31),
(33, 3, 'Hard', 32),
(34, 3, 'Hard', 33),
(35, 3, 'Hard', 34),
(36, 3, 'Hard', 35)
ON CONFLICT DO NOTHING;
