-- USER_ITEMS: Karakter envanter tablosu (normalize edilmiş)
-- Kaynak: MSSQL dbo.USERDATA.strItem (binary 616 blob) → normalize tablo
-- MSSQL'de item verileri tek bir binary blob'da saklanıyordu.
-- Her slot: item_id(4) + durability(2) + count(2) = 8 byte × 77 slot = 616 byte
CREATE TABLE IF NOT EXISTS user_items (
    id              BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    str_user_id     VARCHAR(21)     NOT NULL,
    slot_index      SMALLINT        NOT NULL,
    item_id         INTEGER         NOT NULL DEFAULT 0,
    durability      SMALLINT        NOT NULL DEFAULT 0,
    count           SMALLINT        NOT NULL DEFAULT 0,
    flag            SMALLINT        NOT NULL DEFAULT 0,
    serial_num      BIGINT          NOT NULL DEFAULT 0,
    expire_time     INTEGER         NOT NULL DEFAULT 0,

    UNIQUE (str_user_id, slot_index)
);

CREATE INDEX IF NOT EXISTS idx_user_items_user ON user_items (str_user_id);
CREATE INDEX IF NOT EXISTS idx_user_items_item ON user_items (item_id);

-- USER_DELETED_ITEMS: Silinen item geçmişi (normalize edilmiş)
-- Kaynak: MSSQL dbo.USERDATA.strDeletedItem (binary 480 blob)
CREATE TABLE IF NOT EXISTS user_deleted_items (
    id              BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    str_user_id     VARCHAR(21)     NOT NULL,
    slot_index      SMALLINT        NOT NULL,
    item_id         INTEGER         NOT NULL DEFAULT 0,
    durability      SMALLINT        NOT NULL DEFAULT 0,
    count           SMALLINT        NOT NULL DEFAULT 0,
    deleted_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_user_deleted_items_user ON user_deleted_items (str_user_id);
