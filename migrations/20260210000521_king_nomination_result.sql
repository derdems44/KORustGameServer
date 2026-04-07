-- King election nomination results — historical election records.
-- Source: MSSQL KING_NOMINATION_RESULT (0 rows, schema only)

CREATE TABLE IF NOT EXISTS king_nomination_result (
    id          SERIAL   PRIMARY KEY,
    nation      SMALLINT NOT NULL,
    user_id     VARCHAR(21) NOT NULL,
    rank        SMALLINT NOT NULL DEFAULT 0,
    clan_id     SMALLINT NOT NULL DEFAULT 0,
    month       SMALLINT NOT NULL,
    year        SMALLINT NOT NULL,
    king_votes  INTEGER  NOT NULL DEFAULT 0,
    total_votes INTEGER  NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_king_nomination_nation_year
    ON king_nomination_result (nation, year, month);
