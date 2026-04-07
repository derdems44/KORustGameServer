-- User Costume state persistence (v2525 WIZ_COSTUME 0xC3).
-- Stores the currently equipped costume appearance per character.

CREATE TABLE IF NOT EXISTS user_costume (
    character_id VARCHAR(21) PRIMARY KEY,
    active_type  SMALLINT NOT NULL DEFAULT 0,  -- 0=none, 1=available, 2=equipped, 3=expired
    item_id      INTEGER  NOT NULL DEFAULT 0,  -- equipped costume item ID
    item_param   INTEGER  NOT NULL DEFAULT 0,  -- costume item parameter
    scale_raw    INTEGER  NOT NULL DEFAULT 0,  -- model scale value
    color_index  SMALLINT NOT NULL DEFAULT 0,  -- dye color index (0-13)
    expiry_time  BIGINT   NOT NULL DEFAULT 0   -- absolute UNIX expiry timestamp (seconds)
);
