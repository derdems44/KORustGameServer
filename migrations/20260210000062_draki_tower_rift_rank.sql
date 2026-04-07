-- Draki Tower rift ranking leaderboard.
-- C++ Reference: DRAKI_TOWER_RIFT_RANK table in MSSQL

CREATE TABLE IF NOT EXISTS draki_tower_rift_rank (
    s_index     SERIAL PRIMARY KEY,
    class       INTEGER NOT NULL DEFAULT 0,
    class_name  VARCHAR(50) NOT NULL DEFAULT '',
    rank_id     INTEGER NOT NULL DEFAULT 0,
    str_user_id VARCHAR(21) NOT NULL DEFAULT '',
    b_stage     SMALLINT NOT NULL DEFAULT 0,
    finish_time INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_draki_rift_rank_class ON draki_tower_rift_rank(class);
CREATE INDEX IF NOT EXISTS idx_draki_rift_rank_user ON draki_tower_rift_rank(str_user_id);
