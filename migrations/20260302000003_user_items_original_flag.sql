-- Add original_flag column to user_items for seal state preservation.
-- C++ Reference: _ITEM_DATA::oFlag — stores pre-seal flag for restoration on unseal.
-- Also added to warehouse/vip_warehouse/clan_warehouse for consistency.

ALTER TABLE user_items ADD COLUMN IF NOT EXISTS original_flag SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE user_warehouse ADD COLUMN IF NOT EXISTS original_flag SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE vip_warehouse_items ADD COLUMN IF NOT EXISTS original_flag SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE clan_warehouse_items ADD COLUMN IF NOT EXISTS original_flag SMALLINT NOT NULL DEFAULT 0;
