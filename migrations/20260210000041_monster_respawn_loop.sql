-- Monster respawn loop — chain respawn definitions.
-- Source: MSSQL MONSTER_RESPAWNLOOP_LIST (12 rows).
--
-- When monster `idead` dies, monster `iborn` spawns after `deadtime` seconds.
-- Two chains:
--   Circular: 8950 → 8951 → 8952 → 8953 → 8954 → 8955 → 8956 → 8950 (5s each)
--   Terminal: 9307 → 9308 → 9309 → 9310 → 9311 → 8998 (60s each)
-- `count` = how many of iborn to spawn, `stable` = always true in data.

CREATE TABLE IF NOT EXISTS monster_respawn_loop (
    idead     SMALLINT NOT NULL,
    iborn     SMALLINT NOT NULL,
    stable    BOOLEAN  NOT NULL DEFAULT TRUE,
    count     SMALLINT NOT NULL DEFAULT 1,
    deadtime  SMALLINT NOT NULL DEFAULT 5,
    PRIMARY KEY (idead)
);

INSERT INTO monster_respawn_loop (idead, iborn, stable, count, deadtime) VALUES
(8950, 8951, TRUE, 1,  5),
(8951, 8952, TRUE, 1,  5),
(8952, 8953, TRUE, 1,  5),
(8953, 8954, TRUE, 1,  5),
(8954, 8955, TRUE, 1,  5),
(8955, 8956, TRUE, 1,  5),
(8956, 8950, TRUE, 1,  5),
(9307, 9308, TRUE, 1, 60),
(9308, 9309, TRUE, 1, 60),
(9309, 9310, TRUE, 1, 60),
(9310, 9311, TRUE, 1, 60),
(9311, 8998, TRUE, 5, 60)
ON CONFLICT DO NOTHING;
