-- Cinderella War (Fun Class) event settings.
-- C++ Reference: CINDWAR_SETTING table, 5 tier presets.
-- Each setting defines playtime, preparation, level range, zone, etc.

CREATE TABLE IF NOT EXISTS cindwar_setting (
    setting_id  SMALLINT    NOT NULL PRIMARY KEY,
    playtime    INTEGER     NOT NULL DEFAULT 0,
    preparetime INTEGER     NOT NULL DEFAULT 0,
    min_level   SMALLINT    NOT NULL DEFAULT 0,
    max_level   SMALLINT    NOT NULL DEFAULT 0,
    req_money   INTEGER     NOT NULL DEFAULT 0,
    req_loyalty INTEGER     NOT NULL DEFAULT 0,
    max_user_limit SMALLINT NOT NULL DEFAULT 1,
    zone_id     SMALLINT    NOT NULL DEFAULT 21,
    beginner_level SMALLINT NOT NULL DEFAULT 1
);

INSERT INTO cindwar_setting (setting_id, playtime, preparetime, min_level, max_level, req_money, req_loyalty, max_user_limit, zone_id, beginner_level) VALUES
(0, 60, 0, 1, 90, 0, 0, 0, 110, 65),
(1, 60, 0, 1, 90, 0, 0, 0, 110, 47),
(2, 60, 0, 1, 90, 0, 0, 0, 110, 59),
(3, 60, 0, 1, 90, 0, 0, 0, 110, 83),
(4, 60, 0, 1, 90, 0, 0, 0, 110, 90)
ON CONFLICT DO NOTHING;
