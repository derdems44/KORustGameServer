-- Monster Boss Random Stages
-- Source: MSSQL MONSTER_BOSS_RANDOM_STAGES (33 rows).
-- Defines which boss monsters spawn at random in specific zones.
-- C++ Reference: CGameServerDlg::LoadMonsterBossRandomStages()

CREATE TABLE IF NOT EXISTS monster_boss_random_stages (
    stage        SMALLINT NOT NULL,
    monster_id   SMALLINT NOT NULL,
    monster_zone SMALLINT NOT NULL,
    monster_name VARCHAR(250) NOT NULL DEFAULT '',
    PRIMARY KEY (stage)
);

INSERT INTO monster_boss_random_stages (stage, monster_id, monster_zone, monster_name) VALUES
(1, 111, 21, 'Kekurikekukaka'),
(2, 906, 1, 'Antares'),
(3, 1400, 1, 'Attila'),
(4, 1005, 1, 'Hyde'),
(5, 906, 2, 'Antares'),
(6, 1400, 2, 'Attila'),
(7, 1005, 2, 'Hyde'),
(8, 607, 71, 'Barrkk'),
(9, 608, 71, 'Barkirra'),
(10, 1106, 71, 'bone collecter'),
(11, 1205, 71, 'Duke'),
(12, 1206, 71, 'Bach'),
(13, 1207, 71, 'Bishop'),
(14, 1315, 71, 'Samma'),
(15, 506, 71, 'Lobo'),
(16, 507, 71, 'Lupus'),
(17, 508, 71, 'Lycaon'),
(18, 1305, 71, 'Javana'),
(19, 908, 71, 'Lesath'),
(20, 907, 71, 'Shaula'),
(21, 2817, 71, 'Orc Bandit Leader'),
(22, 2205, 11, 'Harpy Queen'),
(23, 907, 11, 'Shaula'),
(24, 1306, 11, 'Samma'),
(25, 2005, 11, 'Snake Queen'),
(26, 1107, 11, 'Dragon Tooth'),
(27, 2405, 11, 'Talos'),
(28, 2105, 11, 'Deruvish Founder'),
(29, 1725, 11, 'Troll King'),
(30, 8999, 71, 'Devil Stone'),
(31, 8999, 11, 'Devil Stone'),
(32, 8999, 1, 'Devil Stone'),
(33, 8999, 2, 'Devil Stone')
ON CONFLICT (stage) DO NOTHING;
