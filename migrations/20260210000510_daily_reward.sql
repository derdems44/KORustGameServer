-- Daily login rewards (normalized from MSSQL's 25-column single row)
-- Source: MSSQL DAILY_REWARD (25 cols, 1 row) + DAILY_REWARD_CUMULATIVE (3 cols, 1 row)
CREATE TABLE IF NOT EXISTS daily_reward (
    day_index   SMALLINT PRIMARY KEY CHECK (day_index BETWEEN 0 AND 24),
    item_id     INTEGER NOT NULL
);

INSERT INTO daily_reward (day_index, item_id) VALUES
(0,  900145000),
(1,  900146000),
(2,  910252000),
(3,  910250000),
(4,  910249000),
(5,  910251000),
(6,  910938000),
(7,  700085000),
(8,  900015000),
(9,  900145000),
(10, 900146000),
(11, 910252000),
(12, 811095000),
(13, 910938000),
(14, 700085000),
(15, 900015000),
(16, 900145000),
(17, 900146000),
(18, 811101000),
(19, 700089000),
(20, 931773000),
(21, 910251000),
(22, 900015000),
(23, 910925000),
(24, 900175000)
ON CONFLICT (day_index) DO NOTHING;

CREATE TABLE IF NOT EXISTS daily_reward_cumulative (
    id      INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),  -- singleton
    item1   INTEGER,
    item2   INTEGER,
    item3   INTEGER
);

INSERT INTO daily_reward_cumulative (id, item1, item2, item3)
VALUES (1, 347000000, 347000000, 347000000)
ON CONFLICT (id) DO NOTHING;
