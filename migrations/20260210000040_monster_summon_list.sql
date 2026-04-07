-- Monster summon list — defines summonable monsters (scrolls/stones).
-- Source: MSSQL MONSTER_SUMMON_LIST (27 rows).
--
-- bType: 1 = standard summon, 2 = special summon (Hell Fire, Enigma, etc.)
-- sProbability: weight for random selection (higher = more likely)

CREATE TABLE IF NOT EXISTS monster_summon_list (
    s_sid        SMALLINT    NOT NULL,
    str_name     VARCHAR(30) NOT NULL DEFAULT '',
    s_level      SMALLINT    NOT NULL DEFAULT 0,
    s_probability SMALLINT   NOT NULL DEFAULT 0,
    b_type       SMALLINT    NOT NULL DEFAULT 1,
    PRIMARY KEY (s_sid)
);

INSERT INTO monster_summon_list (s_sid, str_name, s_level, s_probability, b_type) VALUES
(506,  'Lobo',               45,  650,  1),
(507,  'Lupus',              50,  650,  1),
(508,  'Lycaon',             55,  600,  1),
(607,  'Barrkk',             55,  600,  1),
(608,  'Barkirra',           60,  600,  1),
(906,  'Antares',            50,  600,  1),
(907,  'Shaula',             75,  600,  1),
(908,  'Lesath',             75,  600,  1),
(1005, 'Hyde',               45,  650,  1),
(1106, 'bone collecter',    300,  600,  1),
(1107, 'Dragon tooth',       80,  600,  1),
(1205, 'Duke',               55,  300,  1),
(1206, 'Bach',               60,  300,  1),
(1207, 'Bishop',             65,  300,  1),
(1306, 'Samma',              90,  600,  1),
(1400, 'attila',             85,  200,  1),
(1725, 'Troll King',        110,   50,  1),
(2005, 'Snake Queen',       100,   50,  1),
(2105, 'Deruvish founder',  100,   50,  1),
(2205, 'Harpy Queen',       110,   50,  1),
(2405, 'Talos',             140,   50,  1),
(2817, 'Orc bandit leader', 120,   50,  1),
(8260, 'Javana',             65,  600,  1),
(9589, 'Hell Fire',          75, 1000,  2),
(9590, 'Enigma',             75, 1000,  2),
(9591, 'Havoc',              75, 1000,  2),
(9592, 'Cruel',              75, 1000,  2)
ON CONFLICT DO NOTHING;
