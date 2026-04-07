-- Migration 082: Verify new_upgrade table exists and has data
-- The new_upgrade table was created in migration 019 (item_tables) with 18,148 rows
-- from MSSQL NEW_UPGRADE1 table. This migration adds an index for faster lookups
-- by origin_number, which is the primary lookup key at runtime.

CREATE INDEX IF NOT EXISTS idx_new_upgrade_origin_number
    ON new_upgrade (origin_number);

CREATE INDEX IF NOT EXISTS idx_new_upgrade_req_item
    ON new_upgrade (req_item);
