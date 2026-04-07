-- Chaos Stone event rewards by rank.
-- C++ Reference: EVENT_CHAOS_REWARDS table.
-- 18 rows: items, experience, loyalty, cash, and noah rewards per rank.

CREATE TABLE IF NOT EXISTS event_chaos_rewards (
    rank_id          SMALLINT PRIMARY KEY,
    item_id1         INTEGER NOT NULL DEFAULT 0,
    item_count1      INTEGER NOT NULL DEFAULT 0,
    item_expiration1 INTEGER NOT NULL DEFAULT 0,
    item_id2         INTEGER NOT NULL DEFAULT 0,
    item_count2      INTEGER NOT NULL DEFAULT 0,
    item_expiration2 INTEGER NOT NULL DEFAULT 0,
    item_id3         INTEGER NOT NULL DEFAULT 0,
    item_count3      INTEGER NOT NULL DEFAULT 0,
    item_expiration3 INTEGER NOT NULL DEFAULT 0,
    item_id4         INTEGER NOT NULL DEFAULT 0,
    item_count4      INTEGER NOT NULL DEFAULT 0,
    item_expiration4 INTEGER NOT NULL DEFAULT 0,
    item_id5         INTEGER NOT NULL DEFAULT 0,
    item_count5      INTEGER NOT NULL DEFAULT 0,
    item_expiration5 INTEGER NOT NULL DEFAULT 0,
    experience       INTEGER NOT NULL DEFAULT 0,
    loyalty          INTEGER NOT NULL DEFAULT 0,
    cash             INTEGER NOT NULL DEFAULT 0,
    noah             INTEGER NOT NULL DEFAULT 0
);

INSERT INTO event_chaos_rewards (rank_id, item_id1, item_count1, item_expiration1, item_id2, item_count2, item_expiration2, item_id3, item_count3, item_expiration3, item_id4, item_count4, item_expiration4, item_id5, item_count5, item_expiration5, experience, loyalty, cash, noah) VALUES
(1,  900017000, 3, 0, 389196000, 50, 0, 389197000, 50, 0, 389198000, 50, 0, 0, 0, 0, 250000000, 2000, 150, 10000000),
(2,  900017000, 2, 0, 389196000, 25, 0, 389197000, 25, 0, 389198000, 25, 0, 0, 0, 0, 200000000, 1500, 100, 10000000),
(3,  900017000, 1, 0, 389196000, 10, 0, 389197000, 10, 0, 389198000, 10, 0, 0, 0, 0, 150000000, 1000, 50,  10000000),
(4,  389301000, 1, 0, 389197000, 25, 0, 389198000, 25, 0, 0, 0, 0, 0, 0, 0, 100000000, 500,  0,   10000000),
(5,  389301000, 1, 0, 389197000, 15, 0, 389198000, 20, 0, 0, 0, 0, 0, 0, 0, 50000000,  250,  0,   10000000),
(6,  389301000, 1, 0, 389197000, 10, 0, 389198000, 15, 0, 0, 0, 0, 0, 0, 0, 25000000,  100,  0,   10000000),
(7,  389301000, 1, 0, 389198000, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(8,  389301000, 1, 0, 389198000, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(9,  389301000, 1, 0, 389198000, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(10, 389301000, 1, 0, 389198000, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(11, 389301000, 1, 0, 389198000, 5,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(12, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(13, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(14, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(15, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(16, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(17, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000),
(18, 389301000, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25000000, 100, 0, 10000000)
ON CONFLICT DO NOTHING;
