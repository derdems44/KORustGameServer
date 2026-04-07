-- Monster Stone Respawn List
-- Source: MSSQL MONSTER_STONE_RESPAWN_LIST (912 rows).
-- Zone 81 spawns for Monster Stone dungeon, grouped by family (1-26).
-- C++ Reference: CGameServerDlg::LoadMonsterStoneRespawnList()

CREATE TABLE IF NOT EXISTS monster_stone_respawn_list (
    s_index     SMALLINT NOT NULL PRIMARY KEY,
    s_sid       SMALLINT NOT NULL,
    b_type      SMALLINT NOT NULL DEFAULT 0,
    str_name    VARCHAR(110) NOT NULL DEFAULT '',
    s_pid       SMALLINT NOT NULL DEFAULT 0,
    zone_id     SMALLINT NOT NULL DEFAULT 0,
    is_boss     BOOLEAN NOT NULL DEFAULT FALSE,
    family      SMALLINT NOT NULL DEFAULT 0,
    s_count     SMALLINT NOT NULL DEFAULT 0,
    by_direction SMALLINT NOT NULL DEFAULT 0,
    x           SMALLINT NOT NULL DEFAULT 0,
    y           SMALLINT NOT NULL DEFAULT 0,
    z           SMALLINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_monster_stone_zone ON monster_stone_respawn_list (zone_id);
CREATE INDEX IF NOT EXISTS idx_monster_stone_family ON monster_stone_respawn_list (family);

-- Data will be loaded via seed script or runtime MSSQL import.
-- The 912 rows define spawn points per family (1-26) in zone 81.
-- Each family has ~35 spawns (7 Gouger-type + 3 Captain + 4 Hyde-type + etc.)
-- with boss entries (is_boss=1) at family boundaries.
