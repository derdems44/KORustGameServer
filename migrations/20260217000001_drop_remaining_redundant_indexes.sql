-- Sprint 159: Drop remaining 4 redundant indexes
-- These are prefix indexes covered by existing PK/unique constraints.
-- (7 were already dropped in migration 20260210000529)

-- 1) idx_user_quest_user(str_user_id) — prefix of user_quest_pkey(str_user_id, quest_id)
DROP INDEX IF EXISTS idx_user_quest_user;

-- 2) idx_user_items_user(str_user_id) — prefix of user_items unique(str_user_id, slot_index)
DROP INDEX IF EXISTS idx_user_items_user;

-- 3) idx_user_warehouse_account(str_account_id) — prefix of user_warehouse unique(str_account_id, slot_index)
DROP INDEX IF EXISTS idx_user_warehouse_account;

-- 4) idx_user_saved_magic_char(character_id) — prefix of user_saved_magic_pkey(character_id, slot)
DROP INDEX IF EXISTS idx_user_saved_magic_char;
