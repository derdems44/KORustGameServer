-- Premium system tables: premium_item_types, premium_item_exp, account_premium
-- C++ Reference: PREMIUM_ITEM, PREMIUM_ITEM_EXP (LoadServerData), ACCOUNT_PREMIUM_DATA (DBAgent)

-- Premium type definitions with bonus percentages
CREATE TABLE IF NOT EXISTS premium_item_types (
    premium_type    SMALLINT    NOT NULL PRIMARY KEY,
    name            VARCHAR(50) NOT NULL DEFAULT '',
    exp_restore_pct DOUBLE PRECISION NOT NULL DEFAULT 0,
    noah_pct        SMALLINT    NOT NULL DEFAULT 0,
    drop_pct        SMALLINT    NOT NULL DEFAULT 0,
    bonus_loyalty   INTEGER     NOT NULL DEFAULT 0,
    repair_disc_pct SMALLINT    NOT NULL DEFAULT 0,
    item_sell_pct   SMALLINT    NOT NULL DEFAULT 0
);

-- Seed data from MSSQL PREMIUM_ITEM (13 rows)
INSERT INTO premium_item_types (premium_type, name, exp_restore_pct, noah_pct, drop_pct, bonus_loyalty, repair_disc_pct, item_sell_pct) VALUES
(1,  'Normal Premium',   0,  0,   0, 0,  0,  0),
(2,  'Ultra Premium',    0,  0,   0, 0,  0,  0),
(3,  'Bronze Premium',  10,  0,   0, 2, 50, 30),
(4,  'Silver Premium',   0,  0,   0, 0,  0,  0),
(5,  'Gold Premium',     2,  5,   1, 4, 50, 50),
(6,  'Prime Premium',    0,  0,   0, 0,  0,  0),
(7,  'Platinum Premium', 2,  5,   1, 5, 50, 30),
(8,  'Royal Premium',    0,  0,   0, 0,  0,  0),
(9,  'Unknown',          0,  0,   0, 0,  0,  0),
(10, 'DISC Premium',     2, 100,  2, 5, 50, 50),
(11, 'EXP Premium',      1,  0,   0, 5, 50, 50),
(12, 'War Premium',      2,  0,   0, 12, 50, 50),
(13, 'Clan Premium',     1,  0,   0, 5, 50, 50)
ON CONFLICT (premium_type) DO NOTHING;

-- Premium XP bonus by level range (per premium type)
-- C++ Reference: PREMIUM_ITEM_EXP — PremiumExpPercent lookup
CREATE TABLE IF NOT EXISTS premium_item_exp (
    n_index     SMALLINT    NOT NULL PRIMARY KEY,
    premium_type SMALLINT   NOT NULL,
    min_level   SMALLINT    NOT NULL DEFAULT 1,
    max_level   SMALLINT    NOT NULL DEFAULT 83,
    s_percent   SMALLINT    NOT NULL DEFAULT 0
);

-- Seed data from MSSQL PREMIUM_ITEM_EXP (14 rows)
INSERT INTO premium_item_exp (n_index, premium_type, min_level, max_level, s_percent) VALUES
(1,  1,  1, 83,   0),
(2,  2,  1, 83,   0),
(3,  3,  1, 83,  20),
(4,  4,  1, 83,  20),
(5,  5,  1, 83,  20),
(6,  6,  1, 83,   0),
(7,  7,  1, 50, 400),
(8,  7, 51, 83, 100),
(9,  8,  1, 83,   0),
(10, 9,  1, 83,   0),
(11, 10, 1, 83,  30),
(12, 11, 1, 83, 100),
(16, 12, 1, 83,   0),
(17, 13, 1, 83,  10)
ON CONFLICT (n_index) DO NOTHING;

-- Per-account premium subscriptions (normalized from ACCOUNT_PREMIUM_DATA blob)
-- Each slot stores a premium type and its Unix expiry timestamp.
CREATE TABLE IF NOT EXISTS account_premium (
    account_id      VARCHAR(21)  NOT NULL,
    slot            SMALLINT     NOT NULL,
    premium_type    SMALLINT     NOT NULL DEFAULT 0,
    expiry_time     INTEGER      NOT NULL DEFAULT 0,
    PRIMARY KEY (account_id, slot)
);

CREATE INDEX IF NOT EXISTS idx_account_premium_account ON account_premium (account_id);
