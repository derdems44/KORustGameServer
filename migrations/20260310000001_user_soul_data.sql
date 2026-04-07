-- User soul data for v2525 WIZ_SOUL (0xC5) panel.
-- 8 categories (cat_id 0-7), each with 3 rank values.
-- 5 slots (slot_id 0-4), each with a stat value.

CREATE TABLE IF NOT EXISTS user_soul_data (
    character_id VARCHAR(21) PRIMARY KEY,

    -- Category 0 (string 43620)
    cat0_v0 SMALLINT NOT NULL DEFAULT 0,
    cat0_v1 SMALLINT NOT NULL DEFAULT 0,
    cat0_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 1 (string 43621)
    cat1_v0 SMALLINT NOT NULL DEFAULT 0,
    cat1_v1 SMALLINT NOT NULL DEFAULT 0,
    cat1_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 2 (string 43622)
    cat2_v0 SMALLINT NOT NULL DEFAULT 0,
    cat2_v1 SMALLINT NOT NULL DEFAULT 0,
    cat2_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 3 (skip in display, but data exists)
    cat3_v0 SMALLINT NOT NULL DEFAULT 0,
    cat3_v1 SMALLINT NOT NULL DEFAULT 0,
    cat3_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 4 (string 43624)
    cat4_v0 SMALLINT NOT NULL DEFAULT 0,
    cat4_v1 SMALLINT NOT NULL DEFAULT 0,
    cat4_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 5 (string 43625)
    cat5_v0 SMALLINT NOT NULL DEFAULT 0,
    cat5_v1 SMALLINT NOT NULL DEFAULT 0,
    cat5_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 6 (string 43626)
    cat6_v0 SMALLINT NOT NULL DEFAULT 0,
    cat6_v1 SMALLINT NOT NULL DEFAULT 0,
    cat6_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Category 7 (string 43619, default/fallback)
    cat7_v0 SMALLINT NOT NULL DEFAULT 0,
    cat7_v1 SMALLINT NOT NULL DEFAULT 0,
    cat7_v2 SMALLINT NOT NULL DEFAULT 0,

    -- Soul slots (0-4, client adds +50 internally)
    slot0 SMALLINT NOT NULL DEFAULT 0,
    slot1 SMALLINT NOT NULL DEFAULT 0,
    slot2 SMALLINT NOT NULL DEFAULT 0,
    slot3 SMALLINT NOT NULL DEFAULT 0,
    slot4 SMALLINT NOT NULL DEFAULT 0
);
