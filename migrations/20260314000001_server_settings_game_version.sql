-- Add game_version column to server_settings.
-- Allows changing the protocol version from DB without recompiling.
ALTER TABLE server_settings
    ADD COLUMN IF NOT EXISTS game_version SMALLINT NOT NULL DEFAULT 2598;
