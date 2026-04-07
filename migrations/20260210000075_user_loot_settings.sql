-- User Loot Settings table
-- Migrated from MSSQL USER_LOOT_SETTINGS
-- Stores per-user auto-loot filter preferences (class filters, type filters, price threshold).
-- All filter values: 1=enabled, 0=disabled.

CREATE TABLE IF NOT EXISTS user_loot_settings (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(50) NOT NULL UNIQUE,
    warrior SMALLINT NOT NULL DEFAULT 1,
    rogue SMALLINT NOT NULL DEFAULT 1,
    mage SMALLINT NOT NULL DEFAULT 1,
    priest SMALLINT NOT NULL DEFAULT 1,
    weapon SMALLINT NOT NULL DEFAULT 1,
    armor SMALLINT NOT NULL DEFAULT 1,
    accessory SMALLINT NOT NULL DEFAULT 1,
    normal SMALLINT NOT NULL DEFAULT 1,
    upgrade SMALLINT NOT NULL DEFAULT 1,
    craft SMALLINT NOT NULL DEFAULT 1,
    rare SMALLINT NOT NULL DEFAULT 1,
    magic SMALLINT NOT NULL DEFAULT 1,
    unique_grade SMALLINT NOT NULL DEFAULT 1,
    consumable SMALLINT NOT NULL DEFAULT 1,
    price INTEGER NOT NULL DEFAULT 0
);
