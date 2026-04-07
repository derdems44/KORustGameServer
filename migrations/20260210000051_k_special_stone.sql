-- K_SPECIAL_STONE: Special stone definitions for chaos stone summons.
-- MSSQL source: K_SPECIAL_STONE (18 rows).
-- C++ Reference: CGameServerDlg::m_SpecialStoneArray
CREATE TABLE IF NOT EXISTS k_special_stone (
    n_index     INTEGER     NOT NULL PRIMARY KEY,
    zone_id     SMALLINT    NOT NULL DEFAULT 0,
    main_npc    INTEGER     NOT NULL DEFAULT 0,
    monster_name VARCHAR(50) NOT NULL DEFAULT '',
    summon_npc  INTEGER     NOT NULL DEFAULT 0,
    summon_count INTEGER    NOT NULL DEFAULT 0,
    status      SMALLINT    NOT NULL DEFAULT 1
);

INSERT INTO k_special_stone (n_index, zone_id, main_npc, monster_name, summon_npc, summon_count, status) VALUES
(35, 71, 8999, '[ Chaos ] Lupus', 8254, 1, 1),
(36, 71, 8999, 'Jersey', 8915, 1, 1),
(37, 71, 8999, 'Skelcrown', 8907, 1, 1),
(38, 71, 8999, '[ Chaos ] Samma', 8259, 1, 1),
(39, 71, 8999, '[ Chaos ] Bach', 8251, 1, 1),
(40, 71, 8999, '[ Chaos ] Barkira', 8258, 1, 1),
(41, 71, 8999, '[ Chaos ] Lobo', 8253, 1, 1),
(42, 71, 8999, '[ Chaos ] Javana', 8260, 1, 1),
(43, 71, 8999, 'Barrkk', 8814, 1, 1),
(44, 71, 8999, '[ Chaos ] Shaula', 8256, 1, 1),
(45, 71, 8999, 'Dulian', 8820, 1, 1),
(46, 71, 8999, '[ Chaos ] Duke', 8250, 1, 1),
(47, 71, 8999, '[ Chaos ] Lycaon', 8255, 1, 1),
(48, 71, 8999, '[ Chaos ] Lesath', 8257, 1, 1),
(49, 71, 8999, 'Raxton', 8908, 1, 1),
(50, 71, 8999, '[ Chaos ] Bishop', 8252, 1, 1),
(51, 71, 8999, 'Query', 8916, 1, 1),
(52, 71, 8999, 'attila', 8917, 1, 1)
ON CONFLICT (n_index) DO NOTHING;
