-- CLAN_WAREHOUSE_ITEMS: Per-clan warehouse storage (shared bank).
-- Source: MSSQL dbo.KNIGHTS — warehouse_data binary blob + n_money (gold in clan bank)
-- C++ Reference: CKnights::m_sClanWarehouseArray[WAREHOUSE_MAX] in Knights.h
-- WAREHOUSE_MAX = 192 (8 pages * 24 items per page)
-- Normalized from binary blob to relational table (same pattern as user_warehouse).
CREATE TABLE IF NOT EXISTS clan_warehouse_items (
    id              BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    clan_id         SMALLINT        NOT NULL,
    slot_index      SMALLINT        NOT NULL,
    item_id         INTEGER         NOT NULL DEFAULT 0,
    durability      SMALLINT        NOT NULL DEFAULT 0,
    count           SMALLINT        NOT NULL DEFAULT 0,
    flag            SMALLINT        NOT NULL DEFAULT 0,
    serial_num      BIGINT          NOT NULL DEFAULT 0,
    expire_time     INTEGER         NOT NULL DEFAULT 0,

    UNIQUE (clan_id, slot_index)
);

CREATE INDEX IF NOT EXISTS idx_clan_warehouse_items_clan ON clan_warehouse_items (clan_id);
