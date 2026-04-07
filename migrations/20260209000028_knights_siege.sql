-- Knights Siege Warfare table
-- C++ Reference: _KNIGHTS_SIEGE_WARFARE struct in GameDefine.h
-- MSSQL Reference: KNIGHTS_SIEGE_WARFARE table (1 row, castle_index=1 for Delos)

CREATE TABLE IF NOT EXISTS knights_siege_warfare (
    s_castle_index      SMALLINT NOT NULL PRIMARY KEY,
    s_master_knights    SMALLINT NOT NULL DEFAULT 0,
    by_siege_type       SMALLINT NOT NULL DEFAULT 0,
    by_war_day          SMALLINT NOT NULL DEFAULT 0,
    by_war_time         SMALLINT NOT NULL DEFAULT 0,
    by_war_minute       SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_1  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_2  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_3  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_4  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_5  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_6  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_7  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_8  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_9  SMALLINT NOT NULL DEFAULT 0,
    s_challenge_list_10 SMALLINT NOT NULL DEFAULT 0,
    by_war_request_day    SMALLINT NOT NULL DEFAULT 0,
    by_war_request_time   SMALLINT NOT NULL DEFAULT 0,
    by_war_request_minute SMALLINT NOT NULL DEFAULT 0,
    by_guerrilla_war_day    SMALLINT NOT NULL DEFAULT 0,
    by_guerrilla_war_time   SMALLINT NOT NULL DEFAULT 0,
    by_guerrilla_war_minute SMALLINT NOT NULL DEFAULT 0,
    str_challenge_list  VARCHAR(50) NOT NULL DEFAULT '',
    s_moradon_tariff    SMALLINT NOT NULL DEFAULT 10,
    s_delos_tariff      SMALLINT NOT NULL DEFAULT 10,
    n_dungeon_charge    INTEGER NOT NULL DEFAULT 0,
    n_moradon_tax       INTEGER NOT NULL DEFAULT 0,
    n_delos_tax         INTEGER NOT NULL DEFAULT 0,
    s_request_list_1    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_2    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_3    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_4    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_5    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_6    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_7    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_8    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_9    SMALLINT NOT NULL DEFAULT 0,
    s_request_list_10   SMALLINT NOT NULL DEFAULT 0
);

-- Seed with default data (castle index 1 = Delos, matching MSSQL data)
INSERT INTO knights_siege_warfare (
    s_castle_index, s_master_knights, by_siege_type,
    s_moradon_tariff, s_delos_tariff
) VALUES (
    1, 0, 1, 10, 10
) ON CONFLICT (s_castle_index) DO NOTHING;
