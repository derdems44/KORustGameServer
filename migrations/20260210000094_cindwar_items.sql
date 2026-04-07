-- Cinderella War (Fun Class) equipment sets per tier.
-- C++ Reference: CINDWAR_ITEMS0 through CINDWAR_ITEMS4 (5 tiers, identical schema).
-- Unified into one table with a `tier` column (0-4).
-- Each tier has 375 rows: 5 classes x 75 inventory slots.

CREATE TABLE IF NOT EXISTS cindwar_items (
    tier        SMALLINT    NOT NULL,
    id          INTEGER     NOT NULL,
    class       SMALLINT    NOT NULL,
    slot_id     SMALLINT    NOT NULL,
    item_id     INTEGER     NOT NULL DEFAULT 0,
    item_count  SMALLINT    NOT NULL DEFAULT 0,
    item_duration SMALLINT  NOT NULL DEFAULT 0,
    item_flag   SMALLINT    NOT NULL DEFAULT 0,
    item_expire INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY (tier, id)
);

CREATE INDEX IF NOT EXISTS idx_cindwar_items_tier_class ON cindwar_items (tier, class);
