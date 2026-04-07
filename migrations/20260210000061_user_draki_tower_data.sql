-- User Draki Tower progress and entrance limit persistence.
-- C++ Reference: USER_DRAKI_TOWER_DATA table in MSSQL

CREATE TABLE IF NOT EXISTS user_draki_tower_data (
    str_user_id     VARCHAR(21) NOT NULL PRIMARY KEY,
    class           INTEGER NOT NULL DEFAULT 0,
    class_name      VARCHAR(50) NOT NULL DEFAULT '',
    i_draki_time    INTEGER NOT NULL DEFAULT 0,
    b_draki_stage   SMALLINT NOT NULL DEFAULT 0,
    b_draki_enterance_limit SMALLINT NOT NULL DEFAULT 0
);
