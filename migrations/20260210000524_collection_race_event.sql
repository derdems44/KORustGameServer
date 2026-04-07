-- Collection Race event configuration and rewards.
-- Source: MSSQL COLLECTION_RACE_EVENT_SETTINGS (4 rows) + COLLECTION_RACE_EVENT_REWARD (9 rows)

CREATE TABLE IF NOT EXISTS collection_race_settings (
    event_index     SMALLINT    NOT NULL,
    event_name      VARCHAR(30) NOT NULL,
    unit1           SMALLINT    NOT NULL DEFAULT 0,
    unit_count1     SMALLINT    NOT NULL DEFAULT 0,
    unit2           SMALLINT    NOT NULL DEFAULT 0,
    unit_count2     SMALLINT    NOT NULL DEFAULT 0,
    unit3           SMALLINT    NOT NULL DEFAULT 0,
    unit_count3     SMALLINT    NOT NULL DEFAULT 0,
    min_level       SMALLINT    NOT NULL DEFAULT 1,
    max_level       SMALLINT    NOT NULL DEFAULT 83,
    event_zone      SMALLINT    NOT NULL DEFAULT 71,
    event_time      INTEGER     NOT NULL DEFAULT 60,
    user_limit      INTEGER     NOT NULL DEFAULT 2500,
    is_repeat       BOOLEAN     NOT NULL DEFAULT FALSE,
    auto_start      BOOLEAN     NOT NULL DEFAULT FALSE,
    auto_hour       SMALLINT    NOT NULL DEFAULT 0,
    auto_minute     SMALLINT    NOT NULL DEFAULT 0,
    PRIMARY KEY (event_index)
);

INSERT INTO collection_race_settings
    (event_index, event_name, unit1, unit_count1, unit2, unit_count2, unit3, unit_count3,
     min_level, max_level, event_zone, event_time, user_limit, is_repeat, auto_start, auto_hour, auto_minute)
VALUES
    (1, 'CR ( CZ MOB)', 8013, 50, 8017, 50, 8851, 50, 1, 83, 71, 60, 2500, TRUE, FALSE, 20, 0),
    (6, 'Collection Race 2x Repeatable', 1, 40, 2221, 20, 2222, 20, 35, 72, 71, 60, 5000, TRUE, TRUE, 16, 0),
    (7, 'CR ( Riote/Atross )', 1311, 10, 1402, 10, 0, 0, 1, 83, 71, 60, 5000, FALSE, FALSE, 17, 0),
    (8, 'CR ( CZ-KC )', 8950, 5, 8951, 5, 1, 20, 1, 83, 71, 60, 2500, TRUE, FALSE, 8, 0)
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS collection_race_reward (
    idx             SMALLINT     NOT NULL,
    event_id        SMALLINT     NOT NULL,
    description     VARCHAR(100) NOT NULL DEFAULT '',
    item_id         INTEGER      NOT NULL DEFAULT 0,
    item_count      INTEGER      NOT NULL DEFAULT 0,
    rate            INTEGER      NOT NULL DEFAULT 100,
    item_time       INTEGER      NOT NULL DEFAULT 0,
    item_flag       INTEGER      NOT NULL DEFAULT 0,
    item_session     INTEGER      NOT NULL DEFAULT 0,
    PRIMARY KEY (idx)
);

INSERT INTO collection_race_reward
    (idx, event_id, description, item_id, item_count, rate, item_time, item_flag, item_session)
VALUES
    (1, 1, 'Silvery Gem', 389196000, 20, 100, 0, 0, 0),
    (2, 1, 'Voucher of Genie ( 7 Day )', 700093000, 1, 75, 0, 0, 0),
    (3, 1, 'Automatic Mining + Automatic Looting', 510000000, 1, 25, 1, 0, 0),
    (19, 6, 'Silvery Gem', 389196000, 10, 100, 0, 0, 0),
    (20, 6, '1000 Knight Cash Certificate', 700085000, 1, 80, 0, 0, 0),
    (34, 7, 'Blue Potion', 900128000, 10, 100, 0, 0, 0),
    (35, 7, 'Blue Potion', 900128000, 10, 100, 0, 0, 0),
    (61, 8, 'Silvery Gem', 389196000, 10, 100, 0, 0, 0),
    (62, 8, '1000 Knight Cash Certificate', 700085000, 1, 80, 0, 0, 0)
ON CONFLICT DO NOTHING;
