-- Migration 037: User saved magic (buff persistence across logout/zone change).
--
-- C++ Reference: USER_SAVED_MAGIC table (MSSQL) — 10 skill/duration pairs per character.
-- Normalized design: one row per saved buff slot instead of wide columns.
-- Max 10 buff slots per character (enforced by application logic).

CREATE TABLE IF NOT EXISTS user_saved_magic (
    character_id VARCHAR(21) NOT NULL,
    slot         SMALLINT    NOT NULL CHECK (slot >= 0 AND slot < 10),
    skill_id     INTEGER     NOT NULL DEFAULT 0,
    remaining_duration INTEGER NOT NULL DEFAULT 0,  -- seconds remaining
    PRIMARY KEY (character_id, slot)
);

CREATE INDEX IF NOT EXISTS idx_user_saved_magic_char
    ON user_saved_magic (character_id);
