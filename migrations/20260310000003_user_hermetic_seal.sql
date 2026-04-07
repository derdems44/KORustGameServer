-- User Hermetic Seal data for v2525 WIZ_ABILITY (0xCF) panel.
-- 24-slot circular wheel, 9 upgrade levels, 2-hour progress timer.
-- Binary/ Reference: SetUserAbility @ 0x1400ab4c0

CREATE TABLE IF NOT EXISTS user_hermetic_seal (
    character_id VARCHAR(21) PRIMARY KEY,

    -- Maximum tier achieved (0-9, controls star display)
    max_tier SMALLINT NOT NULL DEFAULT 0,

    -- Currently selected slot index (0-23)
    selected_slot SMALLINT NOT NULL DEFAULT 0,

    -- Status: 0=active/running, 1=paused, 2=completed
    status SMALLINT NOT NULL DEFAULT 1,

    -- Number of upgrade attempts
    upgrade_count SMALLINT NOT NULL DEFAULT 0,

    -- Current upgrade level (0-9)
    current_level SMALLINT NOT NULL DEFAULT 0,

    -- Elapsed progress time in seconds (stored as real for sub-second precision)
    elapsed_time REAL NOT NULL DEFAULT 0.0
);
