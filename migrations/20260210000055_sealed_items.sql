-- SEALED_ITEMS: Player sealed item records.
-- MSSQL source: SEALED_ITEMS (249 rows, player data).
-- C++ Reference: CGameServerDlg::m_SealedItemArray
CREATE TABLE IF NOT EXISTS sealed_items (
    id              SERIAL      PRIMARY KEY,
    account_id      VARCHAR(21) NOT NULL DEFAULT '',
    character_id    VARCHAR(21) NOT NULL DEFAULT '',
    item_serial     BIGINT      NOT NULL DEFAULT 0,
    item_id         INTEGER     NOT NULL DEFAULT 0,
    seal_type       SMALLINT    NOT NULL DEFAULT 0,
    original_seal_type SMALLINT NOT NULL DEFAULT 0,
    prelock_state   SMALLINT    NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_sealed_items_account ON sealed_items(account_id);
CREATE INDEX IF NOT EXISTS idx_sealed_items_character ON sealed_items(character_id);
CREATE INDEX IF NOT EXISTS idx_sealed_items_serial ON sealed_items(item_serial);
