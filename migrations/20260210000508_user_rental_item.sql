-- Per-user rental item tracking (borrower side)
-- Source: MSSQL USER_RENTAL_ITEM (13 columns, 0 rows - schema only)
-- Note: rental_item table (NPC catalog) already exists from a previous migration
CREATE TABLE IF NOT EXISTS user_rental_item (
    user_id         VARCHAR(21) NOT NULL,
    account_id      VARCHAR(21),
    rental_type     SMALLINT,
    reg_type        SMALLINT,
    rental_index    INTEGER,
    item_index      INTEGER,
    durability      SMALLINT,
    serial_number   BIGINT,
    rental_money    INTEGER,
    rental_time     SMALLINT,
    during_time     SMALLINT,
    rental_at       TIMESTAMPTZ,
    registered_at   TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, rental_index)
);

CREATE INDEX IF NOT EXISTS idx_user_rental_item_account ON user_rental_item(account_id);
