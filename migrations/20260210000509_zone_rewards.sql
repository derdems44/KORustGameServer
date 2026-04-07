-- Zone PvP kill rewards
-- Source: MSSQL ZONE_KILL_REWARD (1 row)
CREATE TABLE IF NOT EXISTS zone_kill_reward (
    idx                 INTEGER PRIMARY KEY,
    zone_id             SMALLINT NOT NULL,
    nation              SMALLINT NOT NULL DEFAULT 0,
    party_required      SMALLINT NOT NULL DEFAULT 0,
    all_party_reward    BOOLEAN NOT NULL DEFAULT false,
    kill_count          SMALLINT NOT NULL DEFAULT 1,
    item_name           VARCHAR(100),
    item_id             INTEGER NOT NULL,
    item_duration       SMALLINT NOT NULL DEFAULT 0,
    item_count          INTEGER NOT NULL DEFAULT 1,
    item_flag           SMALLINT NOT NULL DEFAULT 0,
    item_expiration     INTEGER NOT NULL DEFAULT 0,
    drop_rate           SMALLINT NOT NULL DEFAULT 0,
    give_to_warehouse   BOOLEAN NOT NULL DEFAULT false,
    status              SMALLINT NOT NULL DEFAULT 1,
    is_priest           BOOLEAN NOT NULL DEFAULT false,
    priest_rate         SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO zone_kill_reward (idx, zone_id, nation, party_required, all_party_reward, kill_count, item_name, item_id, item_duration, item_count, item_flag, item_expiration, drop_rate, give_to_warehouse, status, is_priest, priest_rate)
VALUES (1, 71, 0, 2, true, 1, 'Meat Dumpling', 508216000, 1, 1, 0, 0, 3000, false, 1, false, 0)
ON CONFLICT (idx) DO NOTHING;

-- Zone online-time rewards (normal + premium)
-- Source: MSSQL ZONE_ONLINE_REWARD (1 row)
CREATE TABLE IF NOT EXISTS zone_online_reward (
    zone_id         SMALLINT PRIMARY KEY,
    item_id         INTEGER NOT NULL DEFAULT 0,
    item_count      INTEGER NOT NULL DEFAULT 0,
    item_time       INTEGER NOT NULL DEFAULT 0,
    minute          INTEGER NOT NULL DEFAULT 0,
    loyalty         INTEGER NOT NULL DEFAULT 0,
    cash            INTEGER NOT NULL DEFAULT 0,
    tl              INTEGER NOT NULL DEFAULT 0,
    pre_item_id     INTEGER NOT NULL DEFAULT 0,
    pre_item_count  INTEGER NOT NULL DEFAULT 0,
    pre_item_time   INTEGER NOT NULL DEFAULT 0,
    pre_minute      INTEGER NOT NULL DEFAULT 0,
    pre_loyalty     INTEGER NOT NULL DEFAULT 0,
    pre_cash        INTEGER NOT NULL DEFAULT 0,
    pre_tl          INTEGER NOT NULL DEFAULT 0
);

INSERT INTO zone_online_reward (zone_id) VALUES (0)
ON CONFLICT (zone_id) DO NOTHING;
