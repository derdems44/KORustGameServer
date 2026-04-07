-- King System tables
-- C++ Reference: KING_SYSTEM table in MSSQL (30 columns, 2 rows per server)
-- One row per nation (1=Karus, 2=Elmorad)

CREATE TABLE IF NOT EXISTS king_system (
    -- Nation identifier (PK): 1=Karus, 2=Elmorad
    by_nation       SMALLINT NOT NULL PRIMARY KEY,

    -- Election schedule type (C++ ElectionType enum)
    -- 0=NO_TERM, 1=NOMINATION, 2=PRE_ELECTION, 3=ELECTION, 6=TERM_STARTED, 7=TERM_ENDED
    by_type         SMALLINT NOT NULL DEFAULT 0,

    -- Scheduled election date/time
    s_year          SMALLINT NOT NULL DEFAULT 2026,
    by_month        SMALLINT NOT NULL DEFAULT 12,
    by_day          SMALLINT NOT NULL DEFAULT 18,
    by_hour         SMALLINT NOT NULL DEFAULT 0,
    by_minute       SMALLINT NOT NULL DEFAULT 0,

    -- Impeachment state and schedule
    by_im_type      SMALLINT NOT NULL DEFAULT 0,
    s_im_year       SMALLINT NOT NULL DEFAULT 0,
    by_im_month     SMALLINT NOT NULL DEFAULT 0,
    by_im_day       SMALLINT NOT NULL DEFAULT 0,
    by_im_hour      SMALLINT NOT NULL DEFAULT 0,
    by_im_minute    SMALLINT NOT NULL DEFAULT 0,

    -- Noah (coin) bonus event state
    by_noah_event           SMALLINT NOT NULL DEFAULT 0,
    by_noah_event_day       SMALLINT NOT NULL DEFAULT 0,
    by_noah_event_hour      SMALLINT NOT NULL DEFAULT 0,
    by_noah_event_minute    SMALLINT NOT NULL DEFAULT 0,
    s_noah_event_duration   SMALLINT NOT NULL DEFAULT 0,

    -- EXP bonus event state
    by_exp_event            SMALLINT NOT NULL DEFAULT 0,
    by_exp_event_day        SMALLINT NOT NULL DEFAULT 0,
    by_exp_event_hour       SMALLINT NOT NULL DEFAULT 0,
    by_exp_event_minute     SMALLINT NOT NULL DEFAULT 0,
    s_exp_event_duration    SMALLINT NOT NULL DEFAULT 0,

    -- Treasury and tax
    n_tribute               INTEGER NOT NULL DEFAULT 0,
    by_territory_tariff     SMALLINT NOT NULL DEFAULT 0,
    n_territory_tax         INTEGER NOT NULL DEFAULT 0,
    n_national_treasury     INTEGER NOT NULL DEFAULT 0,

    -- Current king
    str_king_name           VARCHAR(21) NOT NULL DEFAULT '',
    s_king_clan_id          SMALLINT NOT NULL DEFAULT 0,

    -- Impeachment requester
    str_im_request_id       VARCHAR(21) NOT NULL DEFAULT ''
);

-- Seed with 2 rows (one per nation)
INSERT INTO king_system (by_nation) VALUES (1) ON CONFLICT DO NOTHING;
INSERT INTO king_system (by_nation) VALUES (2) ON CONFLICT DO NOTHING;
