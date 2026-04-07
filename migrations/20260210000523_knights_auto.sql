-- Auto-created clan templates (one per nation).
-- Source: MSSQL KNIGHTS_AUTO (2 rows)
-- Note: k_mark is a binary blob (DXT cape texture, ~2KB). Stored as BYTEA.

CREATE TABLE IF NOT EXISTS knights_auto (
    nation       SMALLINT    NOT NULL,
    status       SMALLINT    NOT NULL DEFAULT 1,
    clan_id      INTEGER     NOT NULL,
    clan_name    VARCHAR(21) NOT NULL,
    flag         SMALLINT    NOT NULL DEFAULT 3,
    account_id   VARCHAR(21) NOT NULL,
    password     VARCHAR(21) NOT NULL DEFAULT '',
    chief        VARCHAR(21) NOT NULL,
    mark         BYTEA,
    mark_len     INTEGER     NOT NULL DEFAULT 0,
    mark_ver     SMALLINT    NOT NULL DEFAULT 0,
    cape_id      SMALLINT    NOT NULL DEFAULT 0,
    cape_r       SMALLINT    NOT NULL DEFAULT 0,
    cape_g       SMALLINT    NOT NULL DEFAULT 0,
    cape_b       SMALLINT    NOT NULL DEFAULT 0,
    clan_notice  TEXT,
    test         SMALLINT    NOT NULL DEFAULT 0,
    PRIMARY KEY (nation)
);
