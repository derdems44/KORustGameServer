-- VIP_WAREHOUSE: Per-account premium vault storage.
-- Source: MSSQL dbo.VIP_WAREHOUSE — binary blob design.
-- PostgreSQL: normalized tables (vip_warehouse metadata + vip_warehouse_items per-slot).
--
-- C++ Reference: globals.h — VIPWAREHOUSE_MAX = 48 slots
-- Password is a 4-digit PIN stored as plaintext (same as C++ reference).

CREATE TABLE IF NOT EXISTS vip_warehouse (
    str_account_id      VARCHAR(21)     NOT NULL PRIMARY KEY,
    password            VARCHAR(4)      NOT NULL DEFAULT '',
    password_request    SMALLINT        NOT NULL DEFAULT 0,
    vault_expiry        INTEGER         NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS vip_warehouse_items (
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

CREATE INDEX IF NOT EXISTS idx_vip_warehouse_items_account ON vip_warehouse_items (str_account_id);
