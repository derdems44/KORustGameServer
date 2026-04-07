-- KNIGHTS: Klan (clan) tablosu
-- Kaynak: MSSQL dbo.KNIGHTS (PK yok, IDNum unique olmalı)
-- Not: CHAR(N) → VARCHAR(N) trailing space prevention için
CREATE TABLE IF NOT EXISTS knights (
    id_num              SMALLINT        NOT NULL PRIMARY KEY,
    flag                SMALLINT        NOT NULL,
    nation              SMALLINT        NOT NULL,
    ranking             SMALLINT        NOT NULL DEFAULT 0,
    id_name             VARCHAR(21)     NOT NULL,
    members             INTEGER         NOT NULL DEFAULT 1,
    chief               VARCHAR(21)     NOT NULL,
    vice_chief_1        VARCHAR(21),
    vice_chief_2        VARCHAR(21),
    vice_chief_3        VARCHAR(21),
    gold                BIGINT          NOT NULL DEFAULT 0,
    domination          SMALLINT        NOT NULL DEFAULT 0,
    points              INTEGER         NOT NULL DEFAULT 0,
    mark                BYTEA           NOT NULL DEFAULT '\x00',
    s_mark_version      SMALLINT        NOT NULL DEFAULT 0,
    s_mark_len          SMALLINT        NOT NULL DEFAULT 0,
    s_cape              SMALLINT        NOT NULL DEFAULT -1,
    b_cape_r            SMALLINT        NOT NULL DEFAULT 0,
    b_cape_g            SMALLINT        NOT NULL DEFAULT 0,
    b_cape_b            SMALLINT        NOT NULL DEFAULT 0,
    s_cast_cape         SMALLINT        NOT NULL DEFAULT -1,
    b_cast_cape_r       SMALLINT        NOT NULL DEFAULT 0,
    b_cast_cape_g       SMALLINT        NOT NULL DEFAULT 0,
    b_cast_cape_b       SMALLINT        NOT NULL DEFAULT 0,
    b_cast_time         INTEGER         NOT NULL DEFAULT 0,
    s_alliance_knights  SMALLINT        NOT NULL DEFAULT 0,
    clan_point_fund     INTEGER         NOT NULL DEFAULT 0,
    str_clan_notice     VARCHAR(128),
    by_siege_flag       SMALLINT        NOT NULL DEFAULT 0,
    n_lose              SMALLINT        NOT NULL DEFAULT 0,
    n_victory           SMALLINT        NOT NULL DEFAULT 0,
    clan_point_method   SMALLINT        NOT NULL DEFAULT 0,
    n_money             INTEGER         NOT NULL DEFAULT 0,
    dw_time             INTEGER         NOT NULL DEFAULT 0,
    warehouse_data      BYTEA           NOT NULL DEFAULT '\x00',
    str_serial          BYTEA           NOT NULL DEFAULT '\x00',
    s_premium_time      INTEGER         NOT NULL DEFAULT 0,
    s_premium_in_use    SMALLINT        NOT NULL DEFAULT 0,
    dt_create_time      TIMESTAMPTZ     DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knights_nation ON knights (nation);
CREATE INDEX IF NOT EXISTS idx_knights_ranking ON knights (ranking);
CREATE INDEX IF NOT EXISTS idx_knights_alliance ON knights (s_alliance_knights);
CREATE UNIQUE INDEX IF NOT EXISTS idx_knights_name ON knights (id_name);
