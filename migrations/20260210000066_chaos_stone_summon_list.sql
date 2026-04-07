-- Chaos Stone monster summon list.
-- C++ Reference: _CHAOS_STONE_SUMMON_LIST struct, loaded from CHAOS_STONE_SUMMON_LIST table.
-- 18 rows: monsters spawned when a chaos stone is destroyed, grouped by zone + family.

CREATE TABLE IF NOT EXISTS chaos_stone_summon_list (
    n_index              INTEGER PRIMARY KEY,
    zone_id              SMALLINT NOT NULL,
    sid                  SMALLINT NOT NULL,
    monster_spawn_family SMALLINT NOT NULL DEFAULT 1
);

INSERT INTO chaos_stone_summon_list (n_index, zone_id, sid, monster_spawn_family) VALUES
(1,  71, 8254, 1),
(2,  71, 8915, 1),
(3,  71, 8907, 1),
(4,  71, 8259, 1),
(5,  71, 8251, 1),
(6,  71, 8258, 1),
(7,  71, 8253, 2),
(8,  71, 8260, 2),
(9,  71, 8814, 2),
(10, 71, 8256, 2),
(11, 71, 8820, 2),
(12, 71, 8250, 2),
(13, 71, 8255, 3),
(14, 71, 8257, 3),
(15, 71, 8908, 3),
(16, 71, 8252, 3),
(17, 71, 8916, 3),
(18, 71, 8917, 3)
ON CONFLICT DO NOTHING;
