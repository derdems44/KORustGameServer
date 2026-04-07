-- Trash item list for repurchase system (NPC trade type 5).
-- Migrated from MSSQL dbo.TRASH_ITEMLIST — stores sold non-countable items
-- that the player can buy back within 72 minutes.
-- C++ Reference: _DELETED_ITEM struct in GameDefine.h, LOAD_TRASH_ITEMLIST proc

CREATE TABLE IF NOT EXISTS trash_item_list (
    id              BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    str_user_id     VARCHAR(21)     NOT NULL,
    item_id         INTEGER         NOT NULL,
    delete_time     INTEGER         NOT NULL,
    duration        SMALLINT        NOT NULL DEFAULT 0,
    count           INTEGER         NOT NULL DEFAULT 1,
    flag            SMALLINT        NOT NULL DEFAULT 0,
    serial_num      BIGINT          NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_trash_item_list_user ON trash_item_list (str_user_id);
