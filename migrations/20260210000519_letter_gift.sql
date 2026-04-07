-- Gift letter templates — automated item gifts via mail.
-- Source: MSSQL LETTER_GIFT (1 row)

CREATE TABLE IF NOT EXISTS letter_gift (
    id              SERIAL       PRIMARY KEY,
    class           SMALLINT     NOT NULL DEFAULT 0,
    gift_type       SMALLINT     NOT NULL DEFAULT 1,
    sender_id       VARCHAR(21)  NOT NULL DEFAULT '',
    item_name       VARCHAR(50)  NOT NULL DEFAULT '',
    item_description VARCHAR(255) NOT NULL DEFAULT '',
    letter_type     SMALLINT     NOT NULL DEFAULT 1,
    item_id         INTEGER      NOT NULL DEFAULT 0,
    item_count      INTEGER      NOT NULL DEFAULT 0,
    item_duration   INTEGER      NOT NULL DEFAULT 0,
    sending_status  SMALLINT     NOT NULL DEFAULT 0,
    expire_time     INTEGER      NOT NULL DEFAULT 0,
    serial_num      VARCHAR(50)  NOT NULL DEFAULT ''
);
