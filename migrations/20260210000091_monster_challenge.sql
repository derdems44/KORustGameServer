-- Monster Challenge configuration
-- Source: MSSQL MONSTER_CHALLENGE (3 rows).
-- Defines level brackets and start times for monster challenge events.
-- C++ Reference: CGameServerDlg::LoadMonsterChallenge()

CREATE TABLE IF NOT EXISTS monster_challenge (
    s_index      SMALLINT NOT NULL PRIMARY KEY,
    b_start_time1 SMALLINT NOT NULL DEFAULT 99,
    b_start_time2 SMALLINT NOT NULL DEFAULT 99,
    b_start_time3 SMALLINT NOT NULL DEFAULT 99,
    b_level_min  SMALLINT NOT NULL DEFAULT 0,
    b_level_max  SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO monster_challenge (s_index, b_start_time1, b_start_time2, b_start_time3, b_level_min, b_level_max) VALUES
(0, 99, 99, 99, 35, 45),
(1, 9, 19, 99, 46, 59),
(2, 3, 22, 99, 60, 83)
ON CONFLICT (s_index) DO NOTHING;
