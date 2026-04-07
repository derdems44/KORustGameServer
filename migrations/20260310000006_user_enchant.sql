-- User Enchant state persistence (v2525 WIZ_ENCHANT 0xCC).
-- Stores per-character weapon/armor enchant progress.

CREATE TABLE IF NOT EXISTS user_enchant (
    character_id   VARCHAR(21) PRIMARY KEY,
    max_star       SMALLINT NOT NULL DEFAULT 0,   -- highest star tier achieved
    enchant_count  SMALLINT NOT NULL DEFAULT 0,   -- total enchant count
    slot_levels    BYTEA    NOT NULL DEFAULT E'\\x0000000000000000', -- 8 bytes, one per slot
    slot_unlocked  BYTEA    NOT NULL DEFAULT E'\\x000000000000000000', -- 9 bytes, unlock flags
    -- Item enchant state
    item_category     SMALLINT NOT NULL DEFAULT 0,
    item_slot_unlock  SMALLINT NOT NULL DEFAULT 0,
    item_markers      BYTEA    NOT NULL DEFAULT E'\\x0000000000'  -- 5 bytes, marker flags
);
