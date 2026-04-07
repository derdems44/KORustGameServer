-- Sprint 381: Safety constraints for data integrity
-- Prevents negative gold, invalid levels, and out-of-range slot indices

-- Gold cannot go negative
ALTER TABLE userdata ADD CONSTRAINT chk_userdata_gold CHECK (gold >= 0);

-- Level must be valid range (1-83)
ALTER TABLE userdata ADD CONSTRAINT chk_userdata_level CHECK (level >= 1 AND level <= 83);

-- Item slot index must be valid (0-76 for inventory)
ALTER TABLE user_items ADD CONSTRAINT chk_user_items_slot CHECK (slot_index >= 0 AND slot_index < 77);

-- Item count cannot be negative
ALTER TABLE user_items ADD CONSTRAINT chk_user_items_count CHECK (count >= 0);

-- Warehouse slot index (0-191, 8 pages x 24 slots)
ALTER TABLE user_warehouse ADD CONSTRAINT chk_user_warehouse_slot CHECK (slot_index >= 0 AND slot_index < 192);

-- Warehouse coins cannot be negative
ALTER TABLE user_warehouse_coins ADD CONSTRAINT chk_warehouse_coins CHECK (coins >= 0);
