-- MAKE_ITEM: Crafting item code lookup table (10,000 rows)
-- Used by NPC loot system (ItemProdution) to map sIndex -> (item_code, item_level)
-- MSSQL source: MAKE_ITEM (10,000 rows, sIndex 1..10000)
-- C++ Reference: CNpc::ItemProdution() in Npc.cpp

CREATE TABLE IF NOT EXISTS make_item (
    s_index     SMALLINT  NOT NULL PRIMARY KEY,
    item_code   INTEGER   NOT NULL DEFAULT 0,
    item_level  SMALLINT  NOT NULL DEFAULT 0
);

-- Seed data: 10,000 rows generated from MSSQL MAKE_ITEM table.
-- Data is organized as contiguous ranges of (item_code, item_level) pairs.
-- Using generate_series for compact representation of repeated values.

-- Range helper: inserts a range [start..end] with given item_code and item_level
-- Section 1: sIndex 1..147 (weapon/armor base ranges)
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 410, 15 FROM generate_series(1, 64) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 510, 20 FROM generate_series(65, 97) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 610, 25 FROM generate_series(98, 117) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 710, 30 FROM generate_series(118, 129) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 810, 35 FROM generate_series(130, 137) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 910, 40 FROM generate_series(138, 141) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1010, 45 FROM generate_series(142, 144) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1110, 50 FROM generate_series(145, 146) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1210, 55 FROM generate_series(147, 147) AS s
ON CONFLICT (s_index) DO NOTHING;

-- Section 2: sIndex 148..294 (repeat of section 1 pattern)
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 410, 15 FROM generate_series(148, 211) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 510, 20 FROM generate_series(212, 244) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 610, 25 FROM generate_series(245, 264) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 710, 30 FROM generate_series(265, 276) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 810, 35 FROM generate_series(277, 284) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 910, 40 FROM generate_series(285, 288) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1010, 45 FROM generate_series(289, 291) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1110, 50 FROM generate_series(292, 293) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1210, 55 FROM generate_series(294, 294) AS s
ON CONFLICT (s_index) DO NOTHING;

-- Section 3: sIndex 295..4000 (accessory/material ranges)
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 110, 0 FROM generate_series(295, 2147) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 210, 5 FROM generate_series(2148, 3453) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 310, 10 FROM generate_series(3454, 3853) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 410, 15 FROM generate_series(3854, 3917) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 510, 20 FROM generate_series(3918, 3950) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 610, 25 FROM generate_series(3951, 3970) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 710, 30 FROM generate_series(3971, 3982) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 810, 35 FROM generate_series(3983, 3990) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 910, 40 FROM generate_series(3991, 3994) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1010, 45 FROM generate_series(3995, 3997) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1110, 50 FROM generate_series(3998, 3999) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1210, 55 FROM generate_series(4000, 4000) AS s
ON CONFLICT (s_index) DO NOTHING;

-- Section 4: sIndex 4001..8000 (rare/unique item ranges)
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1, 0 FROM generate_series(4001, 6000) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 2, 9 FROM generate_series(6001, 7462) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 3, 18 FROM generate_series(7463, 7862) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 4, 27 FROM generate_series(7863, 7962) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 5, 36 FROM generate_series(7963, 7992) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 6, 45 FROM generate_series(7993, 7998) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 7, 54 FROM generate_series(7999, 8000) AS s
ON CONFLICT (s_index) DO NOTHING;

-- Section 5: sIndex 8001..10000 (enhanced item ranges)
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 110, 0 FROM generate_series(8001, 9000) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 210, 6 FROM generate_series(9001, 9500) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 310, 12 FROM generate_series(9501, 9750) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 410, 18 FROM generate_series(9751, 9875) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 510, 24 FROM generate_series(9876, 9950) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 610, 30 FROM generate_series(9951, 9979) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 710, 36 FROM generate_series(9980, 9991) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 810, 42 FROM generate_series(9992, 9996) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 910, 48 FROM generate_series(9997, 9999) AS s
ON CONFLICT (s_index) DO NOTHING;
INSERT INTO make_item (s_index, item_code, item_level)
SELECT s, 1010, 54 FROM generate_series(10000, 10000) AS s
ON CONFLICT (s_index) DO NOTHING;

CREATE INDEX IF NOT EXISTS idx_make_item_code ON make_item (item_code);
