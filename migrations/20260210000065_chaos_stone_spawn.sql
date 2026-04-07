-- Chaos Stone spawn point definitions.
-- C++ Reference: _CHAOS_STONE_RESPAWN struct, loaded from CHAOS_STONE_SPAWN table.
-- 12 rows: stone spawn positions in zones 71 (Ronark Land), 72 (Ardream), 73 (unused).

CREATE TABLE IF NOT EXISTS chaos_stone_spawn (
    s_index       SMALLINT PRIMARY KEY,
    zone_id       SMALLINT NOT NULL,
    is_open       BOOLEAN NOT NULL DEFAULT FALSE,
    rank          SMALLINT NOT NULL DEFAULT 1,
    chaos_id      SMALLINT NOT NULL,
    count         SMALLINT NOT NULL DEFAULT 1,
    spawn_x       SMALLINT NOT NULL DEFAULT 0,
    spawn_z       SMALLINT NOT NULL DEFAULT 0,
    spawn_time    SMALLINT NOT NULL DEFAULT 1,
    direction     SMALLINT NOT NULL DEFAULT 0,
    radius_range  SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO chaos_stone_spawn (s_index, zone_id, is_open, rank, chaos_id, count, spawn_x, spawn_z, spawn_time, direction, radius_range) VALUES
(1,  71, TRUE, 1, 8945, 1, 1017, 948,  1, 0, 0),
(2,  71, TRUE, 2, 8945, 1, 971,  1007, 3, 0, 0),
(3,  71, TRUE, 3, 8945, 1, 1012, 1055, 5, 0, 0),
(4,  71, TRUE, 4, 8945, 1, 1046, 1006, 7, 0, 0),
(5,  72, TRUE, 1, 8946, 1, 524,  542,  1, 0, 0),
(6,  72, TRUE, 2, 8946, 1, 520,  511,  1, 0, 0),
(7,  72, TRUE, 3, 8946, 1, 498,  528,  1, 0, 0),
(8,  72, TRUE, 4, 8946, 1, 544,  524,  1, 0, 0),
(9,  73, FALSE, 1, 8947, 1, 0,    0,    30, 0, 0),
(10, 73, FALSE, 2, 8947, 1, 0,    0,    30, 0, 0),
(11, 73, FALSE, 3, 8947, 1, 0,    0,    45, 0, 0),
(12, 73, FALSE, 4, 8947, 1, 0,    0,    60, 0, 0)
ON CONFLICT DO NOTHING;
