-- Chaos Stone summon stage definitions.
-- C++ Reference: _CHAOS_STONE_STAGE struct, loaded via LOAD_CHAOS_STONE_STAGE.
-- 9 rows: defines which monster families are valid for each zone.

CREATE TABLE IF NOT EXISTS chaos_stone_summon_stage (
    n_index       SMALLINT PRIMARY KEY,
    zone_id       SMALLINT NOT NULL,
    index_family  SMALLINT NOT NULL
);

INSERT INTO chaos_stone_summon_stage (n_index, zone_id, index_family) VALUES
(1, 71, 1),
(2, 71, 2),
(3, 71, 3),
(4, 72, 1),
(5, 72, 2),
(6, 72, 3),
(7, 73, 1),
(8, 73, 2),
(9, 73, 3)
ON CONFLICT DO NOTHING;
