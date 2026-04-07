-- USER_WAREHOUSE: Per-account warehouse storage (inn).
-- Source: MSSQL dbo.WAREHOUSE — WarehouseData (binary 1536 blob), nMoney (gold in bank)
-- MSSQL stores 192 items as binary blob: item_id(4) + durability(2) + count(2) = 8 byte * 192 = 1536
-- PostgreSQL: normalized table (same structure as user_items but keyed by account).
--
-- C++ Reference: globals.h — WAREHOUSE_MAX = 192 (8 pages * 24 items)
CREATE TABLE IF NOT EXISTS user_warehouse (
    id              BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    str_account_id  VARCHAR(21)     NOT NULL,
    slot_index      SMALLINT        NOT NULL,
    item_id         INTEGER         NOT NULL DEFAULT 0,
    durability      SMALLINT        NOT NULL DEFAULT 0,
    count           SMALLINT        NOT NULL DEFAULT 0,
    flag            SMALLINT        NOT NULL DEFAULT 0,
    serial_num      BIGINT          NOT NULL DEFAULT 0,
    expire_time     INTEGER         NOT NULL DEFAULT 0,

    UNIQUE (str_account_id, slot_index)
);

CREATE INDEX IF NOT EXISTS idx_user_warehouse_account ON user_warehouse (str_account_id);

-- Inn coins (gold stored in warehouse), per-account.
-- Source: MSSQL dbo.WAREHOUSE.nMoney
CREATE TABLE IF NOT EXISTS user_warehouse_coins (
    str_account_id  VARCHAR(21)     NOT NULL PRIMARY KEY,
    coins           INTEGER         NOT NULL DEFAULT 0
);
