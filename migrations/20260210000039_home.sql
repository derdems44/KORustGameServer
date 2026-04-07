-- Per-nation home/respawn coordinates by zone type.
-- Source: MSSQL HOME table (2 rows: nation 1=Karus, 2=Elmorad)
--
-- Each column pair (X, Z) defines a respawn coordinate.
-- LX/LZ are the randomization ranges added to the base position.

CREATE TABLE IF NOT EXISTS home (
    nation              SMALLINT NOT NULL PRIMARY KEY,
    -- Elmorad home zone coordinates
    elmo_zone_x         SMALLINT NOT NULL DEFAULT 0,
    elmo_zone_z         SMALLINT NOT NULL DEFAULT 0,
    elmo_zone_lx        SMALLINT NOT NULL DEFAULT 0,
    elmo_zone_lz        SMALLINT NOT NULL DEFAULT 0,
    -- Karus home zone coordinates
    karus_zone_x        SMALLINT NOT NULL DEFAULT 0,
    karus_zone_z        SMALLINT NOT NULL DEFAULT 0,
    karus_zone_lx       SMALLINT NOT NULL DEFAULT 0,
    karus_zone_lz       SMALLINT NOT NULL DEFAULT 0,
    -- Free (Moradon/neutral) zone coordinates
    free_zone_x         SMALLINT NOT NULL DEFAULT 0,
    free_zone_z         SMALLINT NOT NULL DEFAULT 0,
    free_zone_lx        SMALLINT NOT NULL DEFAULT 0,
    free_zone_lz        SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 1 coordinates
    battle_zone_x       SMALLINT NOT NULL DEFAULT 0,
    battle_zone_z       SMALLINT NOT NULL DEFAULT 0,
    battle_zone_lx      SMALLINT NOT NULL DEFAULT 0,
    battle_zone_lz      SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 2 coordinates
    battle_zone2_x      SMALLINT NOT NULL DEFAULT 0,
    battle_zone2_z      SMALLINT NOT NULL DEFAULT 0,
    battle_zone2_lx     SMALLINT NOT NULL DEFAULT 0,
    battle_zone2_lz     SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 3 coordinates
    battle_zone3_x      SMALLINT NOT NULL DEFAULT 0,
    battle_zone3_z      SMALLINT NOT NULL DEFAULT 0,
    battle_zone3_lx     SMALLINT NOT NULL DEFAULT 0,
    battle_zone3_lz     SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 4 coordinates
    battle_zone4_x      SMALLINT NOT NULL DEFAULT 0,
    battle_zone4_z      SMALLINT NOT NULL DEFAULT 0,
    battle_zone4_lx     SMALLINT NOT NULL DEFAULT 0,
    battle_zone4_lz     SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 5 coordinates
    battle_zone5_x      SMALLINT NOT NULL DEFAULT 0,
    battle_zone5_z      SMALLINT NOT NULL DEFAULT 0,
    battle_zone5_lx     SMALLINT NOT NULL DEFAULT 0,
    battle_zone5_lz     SMALLINT NOT NULL DEFAULT 0,
    -- Battle zone 6 coordinates
    battle_zone6_x      SMALLINT NOT NULL DEFAULT 0,
    battle_zone6_z      SMALLINT NOT NULL DEFAULT 0,
    battle_zone6_lx     SMALLINT NOT NULL DEFAULT 0,
    battle_zone6_lz     SMALLINT NOT NULL DEFAULT 0
);

-- Seed with actual MSSQL data
-- Nation 1 = Karus
INSERT INTO home (
    nation,
    elmo_zone_x, elmo_zone_z, elmo_zone_lx, elmo_zone_lz,
    karus_zone_x, karus_zone_z, karus_zone_lx, karus_zone_lz,
    free_zone_x, free_zone_z, free_zone_lx, free_zone_lz,
    battle_zone_x, battle_zone_z, battle_zone_lx, battle_zone_lz,
    battle_zone2_x, battle_zone2_z, battle_zone2_lx, battle_zone2_lz,
    battle_zone3_x, battle_zone3_z, battle_zone3_lx, battle_zone3_lz,
    battle_zone4_x, battle_zone4_z, battle_zone4_lx, battle_zone4_lz,
    battle_zone5_x, battle_zone5_z, battle_zone5_lx, battle_zone5_lz,
    battle_zone6_x, battle_zone6_z, battle_zone6_lx, battle_zone6_lz
) VALUES (
    1,
    219, 1859, 15, 15,
    441, 1625, 10, 10,
    1380, 1090, 10, 10,
    820, 98, 5, 5,
    61, 158, 5, 5,
    176, 72, 5, 5,
    76, 729, 5, 5,
    76, 729, 5, 5,
    76, 729, 5, 5
) ON CONFLICT (nation) DO NOTHING;

-- Nation 2 = Elmorad
INSERT INTO home (
    nation,
    elmo_zone_x, elmo_zone_z, elmo_zone_lx, elmo_zone_lz,
    karus_zone_x, karus_zone_z, karus_zone_lx, karus_zone_lz,
    free_zone_x, free_zone_z, free_zone_lx, free_zone_lz,
    battle_zone_x, battle_zone_z, battle_zone_lx, battle_zone_lz,
    battle_zone2_x, battle_zone2_z, battle_zone2_lx, battle_zone2_lz,
    battle_zone3_x, battle_zone3_z, battle_zone3_lx, battle_zone3_lz,
    battle_zone4_x, battle_zone4_z, battle_zone4_lx, battle_zone4_lz,
    battle_zone5_x, battle_zone5_z, battle_zone5_lx, battle_zone5_lz,
    battle_zone6_x, battle_zone6_z, battle_zone6_lx, battle_zone6_lz
) VALUES (
    2,
    1595, 412, 15, 15,
    1859, 170, 10, 10,
    630, 920, 10, 10,
    113, 771, 5, 5,
    960, 883, 5, 5,
    824, 924, 5, 5,
    244, 945, 5, 5,
    244, 945, 5, 5,
    244, 945, 5, 5
) ON CONFLICT (nation) DO NOTHING;
