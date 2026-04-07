-- Castellan clan cape bonus data (Castle Siege winner reward).
-- Source: MSSQL KNIGHTS_CASTELLAN (0 rows, schema only)

CREATE TABLE IF NOT EXISTS knights_castellan (
    id_num         SMALLINT NOT NULL,
    cape           SMALLINT NOT NULL DEFAULT -1,
    cape_r         SMALLINT NOT NULL DEFAULT 0,
    cape_g         SMALLINT NOT NULL DEFAULT 0,
    cape_b         SMALLINT NOT NULL DEFAULT 0,
    is_active      BOOLEAN  NOT NULL DEFAULT FALSE,
    remaining_time BIGINT   NOT NULL DEFAULT 0,
    PRIMARY KEY (id_num)
);
