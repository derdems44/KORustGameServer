-- Add patch download URL fields to server_settings.
-- Used by login server to tell launcher where to download patches.

ALTER TABLE server_settings
    ADD COLUMN IF NOT EXISTS patch_url  VARCHAR(256) NOT NULL DEFAULT 'http://127.0.0.1:8080',
    ADD COLUMN IF NOT EXISTS patch_path VARCHAR(128) NOT NULL DEFAULT '/patches/';
