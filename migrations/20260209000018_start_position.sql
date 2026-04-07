-- Start position tables for spawn/respawn locations per zone.
-- Source: MSSQL START_POSITION2369 (79 rows) + START_POSITION_RANDOM (38 rows)
-- C++ structs: _START_POSITION (GameDefine.h:2091), _START_POSITION_RANDOM (GameDefine.h:4178)
--
-- start_position: Per-zone spawn coordinates for Karus and El Morad nations.
--   Used for: /town command, death respawn, zone change default coords, NativeZoneReturn.
--   Keyed by zone_id. Each row has separate X/Z coords for Karus vs El Morad,
--   optional gate coords, and a random range offset (bRangeX/bRangeZ).
--
-- start_position_random: Random spawn points for special zones (Chaos Dungeon, Bowl events).
--   Multiple points per zone; server picks one at random and applies radius offset.

-- ============================================================
-- Table: start_position
-- ============================================================
CREATE TABLE IF NOT EXISTS start_position (
    zone_id       SMALLINT    NOT NULL PRIMARY KEY,
    karus_x       SMALLINT    NOT NULL DEFAULT 0,
    karus_z       SMALLINT    NOT NULL DEFAULT 0,
    elmorad_x     SMALLINT    NOT NULL DEFAULT 0,
    elmorad_z     SMALLINT    NOT NULL DEFAULT 0,
    karus_gate_x  SMALLINT    NOT NULL DEFAULT 0,
    karus_gate_z  SMALLINT    NOT NULL DEFAULT 0,
    elmo_gate_x   SMALLINT    NOT NULL DEFAULT 0,
    elmo_gate_z   SMALLINT    NOT NULL DEFAULT 0,
    range_x       SMALLINT    NOT NULL DEFAULT 0,
    range_z       SMALLINT    NOT NULL DEFAULT 0
);

COMMENT ON TABLE  start_position IS 'Per-zone spawn/respawn positions for each nation (from MSSQL START_POSITION2369)';
COMMENT ON COLUMN start_position.zone_id      IS 'Zone identifier (FK to zone_info)';
COMMENT ON COLUMN start_position.karus_x      IS 'Karus nation spawn X coordinate';
COMMENT ON COLUMN start_position.karus_z      IS 'Karus nation spawn Z coordinate';
COMMENT ON COLUMN start_position.elmorad_x    IS 'El Morad nation spawn X coordinate';
COMMENT ON COLUMN start_position.elmorad_z    IS 'El Morad nation spawn Z coordinate';
COMMENT ON COLUMN start_position.karus_gate_x IS 'Karus gate X coordinate (used in special zones)';
COMMENT ON COLUMN start_position.karus_gate_z IS 'Karus gate Z coordinate (used in special zones)';
COMMENT ON COLUMN start_position.elmo_gate_x  IS 'El Morad gate X coordinate (used in special zones)';
COMMENT ON COLUMN start_position.elmo_gate_z  IS 'El Morad gate Z coordinate (used in special zones)';
COMMENT ON COLUMN start_position.range_x      IS 'Random offset range in X (spawn at x + rand(0..range_x))';
COMMENT ON COLUMN start_position.range_z      IS 'Random offset range in Z (spawn at z + rand(0..range_z))';

-- ============================================================
-- Table: start_position_random
-- ============================================================
CREATE TABLE IF NOT EXISTS start_position_random (
    id        SERIAL      PRIMARY KEY,
    zone_id   SMALLINT    NOT NULL,
    pos_x     SMALLINT    NOT NULL DEFAULT 0,
    pos_z     SMALLINT    NOT NULL DEFAULT 0,
    radius    SMALLINT    NOT NULL DEFAULT 0
);

CREATE INDEX idx_start_position_random_zone ON start_position_random (zone_id);

COMMENT ON TABLE  start_position_random IS 'Random spawn points for special zones like Chaos Dungeon (from MSSQL START_POSITION_RANDOM)';
COMMENT ON COLUMN start_position_random.zone_id IS 'Zone identifier';
COMMENT ON COLUMN start_position_random.pos_x   IS 'Spawn X coordinate';
COMMENT ON COLUMN start_position_random.pos_z   IS 'Spawn Z coordinate';
COMMENT ON COLUMN start_position_random.radius  IS 'Random radius offset applied to pos_x/pos_z';

-- ============================================================
-- Seed data: start_position (79 rows from MSSQL START_POSITION2369)
-- ============================================================
INSERT INTO start_position (zone_id, karus_x, karus_z, elmorad_x, elmorad_z, karus_gate_x, karus_gate_z, elmo_gate_x, elmo_gate_z, range_x, range_z) VALUES
    (1,   437,  1627, 1869, 172,  0, 0, 0, 0, 5, 5),
    (2,   214,  1862, 1598, 407,  0, 0, 0, 0, 5, 5),
    (5,   437,  1627, 1869, 172,  0, 0, 0, 0, 5, 5),
    (6,   437,  1627, 1869, 172,  0, 0, 0, 0, 5, 5),
    (7,   214,  1862, 1598, 407,  0, 0, 0, 0, 5, 5),
    (8,   214,  1862, 1598, 407,  0, 0, 0, 0, 5, 5),
    (11,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (12,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (13,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (14,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (15,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (16,  526,  540,  526,  540,  0, 0, 0, 0, 3, 3),
    (18,  389,  1591, 1334, 91,   0, 0, 0, 0, 0, 0),
    (21,  816,  532,  816,  532,  0, 0, 0, 0, 10, 10),
    (22,  264,  302,  264,  302,  0, 0, 0, 0, 10, 10),
    (23,  264,  302,  264,  302,  0, 0, 0, 0, 10, 10),
    (24,  264,  302,  264,  302,  0, 0, 0, 0, 10, 10),
    (25,  264,  302,  264,  302,  0, 0, 0, 0, 10, 10),
    (28,  439,  1957, 1641, 375,  0, 0, 0, 0, 0, 0),
    (29,  309,  347,  309,  347,  0, 0, 0, 0, 0, 0),
    (30,  505,  252,  505,  252,  0, 0, 0, 0, 5, 5),
    (31,  76,   729,  244,  945,  0, 0, 0, 0, 5, 5),
    (32,  50,   69,   50,   69,   0, 0, 0, 0, 5, 5),
    (33,  50,   69,   50,   69,   0, 0, 0, 0, 5, 5),
    (34,  109,  21,   109,  21,   0, 0, 0, 0, 5, 5),
    (35,  459,  113,  459,  113,  0, 0, 0, 0, 5, 5),
    (36,  459,  113,  459,  113,  0, 0, 0, 0, 5, 5),
    (48,  120,  115,  120,  115,  0, 0, 0, 0, 5, 5),
    (51,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (52,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (53,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (54,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (55,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (61,  820,  98,   113,  768,  0, 0, 0, 0, 5, 5),
    (62,  63,   159,  960,  884,  0, 0, 0, 0, 5, 5),
    (63,  176,  72,   824,  924,  0, 0, 0, 0, 5, 5),
    (64,  810,  780,  236,  230,  0, 0, 0, 0, 5, 5),
    (65,  331,  544,  289,  915,  331, 544, 289, 915, 10, 10),
    (66,  202,  846,  822,  175,  0, 0, 0, 0, 5, 5),
    (67,  0,    0,    0,    0,    0, 0, 0, 0, 5, 5),
    (68,  459,  113,  459,  113,  0, 0, 0, 0, 5, 5),
    (69,  63,   159,  960,  884,  0, 0, 0, 0, 5, 5),
    (71,  1375, 1098, 622,  898,  0, 0, 0, 0, 5, 5),
    (72,  851,  136,  190,  897,  0, 0, 0, 0, 5, 5),
    (73,  515,  104,  513,  916,  0, 0, 0, 0, 5, 5),
    (74,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (75,  53,   1974, 47,   1830, 0, 0, 0, 0, 5, 5),
    (76,  505,  508,  505,  508,  0, 0, 0, 0, 0, 0),
    (77,  759,  497,  250,  555,  0, 0, 0, 0, 5, 5),
    (78,  1375, 1098, 622,  898,  0, 0, 0, 0, 5, 5),
    (81,  204,  197,  204,  197,  1, 1, 0, 0, 5, 0),
    (82,  204,  197,  204,  197,  1, 1, 0, 0, 5, 0),
    (83,  204,  200,  204,  200,  1, 1, 0, 0, 5, 0),
    (84,  51,   58,   201,  207,  5, 5, 0, 0, 0, 0),
    (85,  126,  130,  126,  130,  0, 0, 0, 0, 10, 30),
    (86,  69,   64,   69,   64,   5, 5, 0, 0, 0, 0),
    (87,  224,  272,  799,  749,  0, 0, 0, 0, 0, 0),
    (88,  0,    0,    0,    0,    0, 0, 0, 0, 0, 0),
    (89,  215,  207,  215,  207,  0, 0, 0, 0, 5, 5),
    (91,  259,  385,  459,  113,  0, 0, 0, 0, 5, 5),
    (92,  150,  150,  150,  150,  0, 0, 0, 0, 5, 5),
    (93,  63,   475,  63,   475,  0, 0, 0, 0, 5, 5),
    (94,  110,  20,   110,  20,   0, 0, 0, 0, 5, 5),
    (95,  40,   451,  40,   451,  0, 0, 0, 0, 0, 0),
    (96,  51,   58,   201,  207,  0, 0, 0, 0, 0, 0),
    (97,  51,   58,   201,  207,  0, 0, 0, 0, 0, 0),
    (98,  51,   58,   201,  207,  0, 0, 0, 0, 0, 0),
    (99,  51,   58,   201,  207,  0, 0, 0, 0, 0, 0),
    (105, 269,  385,  438,  119,  0, 0, 0, 0, 5, 5),
    (106, 1991, 53,   1609, 432,  0, 0, 0, 0, 5, 5),
    (107, 1373, 1098, 622,  899,  0, 0, 0, 0, 5, 5),
    (108, 515,  104,  513,  916,  0, 0, 0, 0, 5, 5),
    (109, 146,  539,  151,  925,  0, 0, 0, 0, 5, 5),
    (110, 254,  78,   257,  435,  0, 0, 0, 0, 5, 5),
    (111, 403,  415,  605,  855,  0, 0, 0, 0, 5, 5),
    (112, 412,  508,  478,  943,  0, 0, 0, 0, 5, 5),
    (113, 963,  1002, 1061, 1008, 0, 0, 0, 0, 5, 5),
    (114, 1024, 809,  1019, 1189, 0, 0, 0, 0, 5, 5),
    (115, 755,  496,  271,  550,  0, 0, 0, 0, 5, 5);

-- ============================================================
-- Seed data: start_position_random (38 rows from MSSQL START_POSITION_RANDOM)
-- ============================================================
INSERT INTO start_position_random (zone_id, pos_x, pos_z, radius) VALUES
    -- Zone 71 (Chaos Dungeon / Bowl event zone) - 22 spawn points
    (71, 909,  1042, 1),
    (71, 933,  991,  1),
    (71, 937,  1051, 1),
    (71, 938,  945,  1),
    (71, 947,  891,  1),
    (71, 983,  900,  1),
    (71, 988,  1011, 1),
    (71, 1005, 1007, 1),
    (71, 1007, 1070, 1),
    (71, 1009, 1043, 1),
    (71, 1013, 1013, 1),
    (71, 1015, 1001, 1),
    (71, 1017, 973,  1),
    (71, 1021, 1004, 1),
    (71, 1031, 917,  1),
    (71, 1037, 1004, 1),
    (71, 1050, 1086, 1),
    (71, 1075, 1093, 1),
    (71, 1092, 950,  1),
    (71, 1119, 1041, 1),
    (71, 1124, 1000, 1),
    (71, 1127, 963,  1),
    -- Zone 85 (Chaos Dungeon variant) - 16 spawn points
    (85, 77,   146,  1),
    (85, 77,   130,  1),
    (85, 77,   112,  1),
    (85, 110,  184,  1),
    (85, 112,  159,  1),
    (85, 114,  101,  1),
    (85, 117,  144,  1),
    (85, 126,  144,  1),
    (85, 126,  97,   1),
    (85, 134,  144,  1),
    (85, 137,  101,  1),
    (85, 137,  159,  1),
    (85, 142,  184,  1),
    (85, 174,  112,  1),
    (85, 174,  129,  1),
    (85, 174,  146,  1);
