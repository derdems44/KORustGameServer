-- JackPot settings table
-- C++ Reference: GameDefine.h:188 — _JACKPOT_SETTING struct
-- iType 0 = EXP jackpot, iType 1 = Noah/gold jackpot
-- rate = chance out of 10000, _1000/_500/_100/_50/_10/_2 = multiplier probability thresholds
CREATE TABLE IF NOT EXISTS jackpot_settings (
    i_type   SMALLINT NOT NULL PRIMARY KEY,  -- 0=EXP, 1=Noah
    rate     SMALLINT NOT NULL DEFAULT 0,
    x_1000   SMALLINT NOT NULL DEFAULT 0,
    x_500    SMALLINT NOT NULL DEFAULT 0,
    x_100    SMALLINT NOT NULL DEFAULT 0,
    x_50     SMALLINT NOT NULL DEFAULT 0,
    x_10     SMALLINT NOT NULL DEFAULT 0,
    x_2      SMALLINT NOT NULL DEFAULT 0
);

-- Seed default rows (disabled by default — rate=0)
INSERT INTO jackpot_settings (i_type, rate, x_1000, x_500, x_100, x_50, x_10, x_2)
VALUES
    (0, 0, 0, 0, 0, 0, 0, 0),  -- EXP jackpot (disabled)
    (1, 0, 0, 0, 0, 0, 0, 0)   -- Noah jackpot (disabled)
ON CONFLICT (i_type) DO NOTHING;
