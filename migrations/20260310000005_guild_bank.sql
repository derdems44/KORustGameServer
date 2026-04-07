-- Guild Bank persistence (v2525 WIZ_GUILD_BANK 0xD0).
-- Clan-level shared storage with tabs, items, and transaction logs.

-- Guild bank settings per clan
CREATE TABLE IF NOT EXISTS guild_bank (
    knights_id   INTEGER PRIMARY KEY,        -- clan ID (FK to knights table)
    gold         BIGINT   NOT NULL DEFAULT 0, -- stored gold
    max_tabs     SMALLINT NOT NULL DEFAULT 1, -- unlocked tab count (1-9)
    permissions  SMALLINT NOT NULL DEFAULT 0  -- default member permission flags
);

-- Guild bank item slots (up to 9 tabs × N items per tab)
CREATE TABLE IF NOT EXISTS guild_bank_item (
    id           SERIAL PRIMARY KEY,
    knights_id   INTEGER  NOT NULL,           -- clan ID
    tab_index    SMALLINT NOT NULL DEFAULT 0,  -- tab (0-8)
    slot_id      INTEGER  NOT NULL DEFAULT 0,  -- slot within tab
    item_id      INTEGER  NOT NULL DEFAULT 0,  -- item template ID
    item_count   SMALLINT NOT NULL DEFAULT 0,  -- stack count
    max_durability SMALLINT NOT NULL DEFAULT 0,
    cur_durability SMALLINT NOT NULL DEFAULT 0,
    flag         SMALLINT NOT NULL DEFAULT 0,  -- item flags
    expiry_time  INTEGER  NOT NULL DEFAULT 0,  -- item expiry (seconds)
    UNIQUE (knights_id, tab_index, slot_id)
);

CREATE INDEX IF NOT EXISTS idx_guild_bank_item_clan ON guild_bank_item (knights_id);

-- Guild bank transaction log
CREATE TABLE IF NOT EXISTS guild_bank_log (
    id           SERIAL PRIMARY KEY,
    knights_id   INTEGER  NOT NULL,           -- clan ID
    character_id VARCHAR(21) NOT NULL,        -- who performed the action
    tab_index    SMALLINT NOT NULL DEFAULT 0,
    item_id      INTEGER  NOT NULL DEFAULT 0,
    quantity     SMALLINT NOT NULL DEFAULT 0,
    price        INTEGER  NOT NULL DEFAULT 0,
    action_type  SMALLINT NOT NULL DEFAULT 0, -- deposit=1, withdraw=2, etc.
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_guild_bank_log_clan ON guild_bank_log (knights_id, created_at DESC);
