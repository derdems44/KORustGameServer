-- Damage balance multipliers (single-row).
-- C++ Reference: _DAMAGE_SETTING struct in GameDefine.h:293
-- Source: MSSQL DAMAGE_SETTINGS table (1 row)
--
-- Column naming: attacker_to_target (e.g., priest_to_warrior).
-- All values are float multipliers applied after base damage calculation.

CREATE TABLE IF NOT EXISTS damage_settings (
    id                  SERIAL PRIMARY KEY,
    -- Class vs class PvP multipliers (attacker -> target)
    priest_to_warrior   REAL NOT NULL DEFAULT 1.0,
    priest_to_mage      REAL NOT NULL DEFAULT 1.0,
    priest_to_priest    REAL NOT NULL DEFAULT 1.0,
    priest_to_rogue     REAL NOT NULL DEFAULT 1.0,
    priest_to_kurian    REAL NOT NULL DEFAULT 1.0,
    warrior_to_rogue    REAL NOT NULL DEFAULT 1.0,
    warrior_to_mage     REAL NOT NULL DEFAULT 1.0,
    warrior_to_warrior  REAL NOT NULL DEFAULT 1.0,
    warrior_to_priest   REAL NOT NULL DEFAULT 1.0,
    warrior_to_kurian   REAL NOT NULL DEFAULT 1.0,
    rogue_to_mage       REAL NOT NULL DEFAULT 1.0,
    rogue_to_warrior    REAL NOT NULL DEFAULT 1.0,
    rogue_to_rogue      REAL NOT NULL DEFAULT 1.0,
    rogue_to_priest     REAL NOT NULL DEFAULT 1.0,
    rogue_to_kurian     REAL NOT NULL DEFAULT 1.0,
    kurian_to_mage      REAL NOT NULL DEFAULT 1.0,
    kurian_to_warrior   REAL NOT NULL DEFAULT 1.0,
    kurian_to_rogue     REAL NOT NULL DEFAULT 1.0,
    kurian_to_priest    REAL NOT NULL DEFAULT 1.0,
    kurian_to_kurian    REAL NOT NULL DEFAULT 1.0,
    mage_to_warrior     REAL NOT NULL DEFAULT 1.0,
    mage_to_mage        REAL NOT NULL DEFAULT 1.0,
    mage_to_priest      REAL NOT NULL DEFAULT 1.0,
    mage_to_rogue       REAL NOT NULL DEFAULT 1.0,
    mage_to_kurian      REAL NOT NULL DEFAULT 1.0,
    -- Monster multipliers
    mon_def             REAL NOT NULL DEFAULT 1.0,
    mon_take_damage     REAL NOT NULL DEFAULT 1.5,
    mage_magic_damage   REAL NOT NULL DEFAULT 0.4,
    -- Item class multipliers
    unique_item         REAL NOT NULL DEFAULT 1.2,
    low_class_item      REAL NOT NULL DEFAULT 1.0,
    middle_class_item   REAL NOT NULL DEFAULT 1.0,
    high_class_item     REAL NOT NULL DEFAULT 1.1,
    rare_item           REAL NOT NULL DEFAULT 1.0,
    magic_item          REAL NOT NULL DEFAULT 1.0,
    -- R-attack damage multiplier (for non-priest above level 30)
    r_damage            REAL NOT NULL DEFAULT 0.9
);

-- Seed with actual MSSQL data (ON CONFLICT for idempotent re-apply)
INSERT INTO damage_settings (
    id,
    priest_to_warrior, priest_to_mage, priest_to_priest, priest_to_rogue, priest_to_kurian,
    warrior_to_rogue, warrior_to_mage, warrior_to_warrior, warrior_to_priest, warrior_to_kurian,
    rogue_to_mage, rogue_to_warrior, rogue_to_rogue, rogue_to_priest, rogue_to_kurian,
    kurian_to_mage, kurian_to_warrior, kurian_to_rogue, kurian_to_priest, kurian_to_kurian,
    mage_to_warrior, mage_to_mage, mage_to_priest, mage_to_rogue, mage_to_kurian,
    mon_def, mon_take_damage, mage_magic_damage,
    unique_item, low_class_item, middle_class_item, high_class_item,
    rare_item, magic_item, r_damage
) VALUES (
    1,
    1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0, 1.0,
    1.0, 1.5, 0.4,
    1.2, 1.0, 1.0, 1.1,
    1.0, 1.0, 0.9
) ON CONFLICT (id) DO NOTHING;
