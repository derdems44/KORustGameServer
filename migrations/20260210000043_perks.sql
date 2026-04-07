-- Perk definitions table (static data, 13 rows)
-- C++ Reference: _PERKS struct in GameDefine.h:551, MSSQL PERKS table
CREATE TABLE IF NOT EXISTS perks (
    p_index     INT NOT NULL PRIMARY KEY,
    status      BOOLEAN NOT NULL DEFAULT TRUE,
    description VARCHAR(50) NOT NULL DEFAULT 'perk',
    perk_count  SMALLINT NOT NULL DEFAULT 0,
    perk_max    SMALLINT NOT NULL DEFAULT 5,
    percentage  BOOLEAN NOT NULL DEFAULT FALSE
);

-- Seed 13 perk definitions from MSSQL
INSERT INTO perks (p_index, status, description, perk_count, perk_max, percentage) VALUES
(0,  TRUE, 'Weight',               150, 5, FALSE),
(1,  TRUE, 'Health',               100, 5, FALSE),
(2,  TRUE, 'Mana',                 200, 5, FALSE),
(3,  TRUE, 'Loyalty',                3, 5, FALSE),
(4,  TRUE, 'Drop',                   2, 3, TRUE),
(5,  TRUE, 'Exp',                    4, 5, TRUE),
(6,  TRUE, 'Coins from Monsters',    3, 5, TRUE),
(7,  TRUE, 'Coins on NPC',           2, 5, TRUE),
(8,  TRUE, 'Upgrade Chance',         1, 5, TRUE),
(9,  TRUE, 'Damage to Monsters',     4, 5, TRUE),
(10, TRUE, 'Damage to Player',       2, 5, TRUE),
(11, TRUE, 'Defence',               20, 5, FALSE),
(12, TRUE, 'Attack',                20, 5, FALSE)
ON CONFLICT (p_index) DO NOTHING;

-- Per-character perk point allocations
-- C++ Reference: _PERKS_DATA struct, MSSQL USER_PERKS table
-- Normalized: one row per character with 13 perk level columns + unspent points
CREATE TABLE IF NOT EXISTS user_perks (
    character_id VARCHAR(21) NOT NULL PRIMARY KEY,
    perk_type0   SMALLINT NOT NULL DEFAULT 0,
    perk_type1   SMALLINT NOT NULL DEFAULT 0,
    perk_type2   SMALLINT NOT NULL DEFAULT 0,
    perk_type3   SMALLINT NOT NULL DEFAULT 0,
    perk_type4   SMALLINT NOT NULL DEFAULT 0,
    perk_type5   SMALLINT NOT NULL DEFAULT 0,
    perk_type6   SMALLINT NOT NULL DEFAULT 0,
    perk_type7   SMALLINT NOT NULL DEFAULT 0,
    perk_type8   SMALLINT NOT NULL DEFAULT 0,
    perk_type9   SMALLINT NOT NULL DEFAULT 0,
    perk_type10  SMALLINT NOT NULL DEFAULT 0,
    perk_type11  SMALLINT NOT NULL DEFAULT 0,
    perk_type12  SMALLINT NOT NULL DEFAULT 0,
    rem_perk     SMALLINT NOT NULL DEFAULT 0
);
