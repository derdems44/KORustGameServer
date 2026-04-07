-- Level merchant EXP reward configuration.
-- Source: MSSQL LEVEL_MERCHANT_REWARDS (1 row)

CREATE TABLE IF NOT EXISTS level_merchant_rewards (
    idx             SMALLINT NOT NULL,
    start_hour      SMALLINT NOT NULL DEFAULT 99,
    start_minute    SMALLINT NOT NULL DEFAULT 99,
    finish_time     SMALLINT NOT NULL DEFAULT 99,
    rate_experience INTEGER  NOT NULL DEFAULT 0,
    exp_minute      INTEGER  NOT NULL DEFAULT 0,
    PRIMARY KEY (idx)
);

INSERT INTO level_merchant_rewards (idx, start_hour, start_minute, finish_time, rate_experience, exp_minute)
VALUES (1, 99, 99, 99, 0, 0)
ON CONFLICT DO NOTHING;
