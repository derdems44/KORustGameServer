-- Burning event rate multipliers by level tier
-- Source: MSSQL BURNING_FEATURES (3 rows)
CREATE TABLE IF NOT EXISTS burning_features (
    burn_level  SMALLINT PRIMARY KEY,  -- 1=low, 2=mid, 3=high
    np_rate     SMALLINT NOT NULL DEFAULT 0,
    money_rate  SMALLINT NOT NULL DEFAULT 0,
    exp_rate    SMALLINT NOT NULL DEFAULT 0,
    drop_rate   SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO burning_features (burn_level, np_rate, money_rate, exp_rate, drop_rate) VALUES
(1, 1, 0, 0, 0),
(2, 3, 0, 0, 0),
(3, 5, 0, 0, 0)
ON CONFLICT (burn_level) DO NOTHING;
