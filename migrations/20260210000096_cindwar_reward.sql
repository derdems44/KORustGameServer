-- Cinderella War (Fun Class) rank-based rewards.
-- C++ Reference: CINDWAR_REWARD table, up to 200 rank positions.
-- Normalized: reward items stored as separate rows in cindwar_reward_item.

CREATE TABLE IF NOT EXISTS cindwar_reward (
    rank_id         SMALLINT    NOT NULL PRIMARY KEY,
    exp_count       INTEGER     NOT NULL DEFAULT 0,
    cash_count      INTEGER     NOT NULL DEFAULT 0,
    loyalty_count   INTEGER     NOT NULL DEFAULT 0,
    money_count     INTEGER     NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS cindwar_reward_item (
    rank_id         SMALLINT    NOT NULL,
    slot            SMALLINT    NOT NULL,
    item_id         INTEGER     NOT NULL DEFAULT 0,
    item_count      INTEGER     NOT NULL DEFAULT 0,
    item_duration   INTEGER     NOT NULL DEFAULT 0,
    item_expiration INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_id, slot)
);

-- Seed reward data (20 ranks from MSSQL)
INSERT INTO cindwar_reward (rank_id, exp_count, cash_count, loyalty_count, money_count) VALUES
(1, 100, 200, 300, 400),
(2, 90, 190, 290, 390),
(3, 80, 180, 280, 380),
(4, 70, 170, 270, 370),
(5, 60, 160, 260, 360),
(6, 0, 0, 0, 0),
(7, 0, 0, 0, 0),
(8, 0, 0, 0, 0),
(9, 0, 0, 0, 0),
(10, 0, 0, 0, 0),
(11, 0, 0, 0, 0),
(12, 0, 0, 0, 0),
(13, 0, 0, 0, 0),
(14, 0, 0, 0, 0),
(15, 0, 0, 0, 0),
(16, 0, 0, 0, 0),
(17, 0, 0, 0, 0),
(18, 0, 0, 0, 0),
(19, 0, 0, 0, 0),
(20, 0, 0, 0, 0)
ON CONFLICT DO NOTHING;

-- Seed reward items (only ranks 1-5 have items)
INSERT INTO cindwar_reward_item (rank_id, slot, item_id, item_count, item_duration, item_expiration) VALUES
(1, 1, 379156000, 3, 1, 0),
(2, 1, 379155000, 2, 1, 0),
(3, 1, 379154000, 1, 1, 0),
(4, 1, 389196000, 1, 1, 0),
(5, 1, 810944000, 1, 1, 0)
ON CONFLICT DO NOTHING;
