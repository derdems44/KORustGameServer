-- PUS purchase/refund tracking table
-- Source: MSSQL PUS_REFUND (162 rows)
-- Tracks all cash shop purchases for history and refund purposes.

CREATE TABLE IF NOT EXISTS pus_refund (
    mserial         BIGINT      NOT NULL PRIMARY KEY,
    account_id      VARCHAR(21) NOT NULL,
    item_id         INTEGER     NOT NULL,
    item_count      SMALLINT    NOT NULL DEFAULT 1,
    item_price      INTEGER     NOT NULL DEFAULT 0,
    buying_time     INTEGER     NOT NULL DEFAULT 0,
    item_duration   SMALLINT    NOT NULL DEFAULT 0,
    buy_type        SMALLINT    NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_pus_refund_account ON pus_refund(account_id);
CREATE INDEX IF NOT EXISTS idx_pus_refund_time    ON pus_refund(buying_time);
