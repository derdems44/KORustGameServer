-- Sprint 160: Drop final 3 redundant indexes
-- These are prefix indexes covered by existing PK/unique constraints.

-- 1) idx_vip_warehouse_items_account(str_account_id) — prefix of unique(str_account_id, slot_index)
DROP INDEX IF EXISTS idx_vip_warehouse_items_account;

-- 2) idx_user_daily_quest_char(character_id) — prefix of pk_user_daily_quest(character_id, quest_id)
DROP INDEX IF EXISTS idx_user_daily_quest_char;

-- 3) idx_daily_reward_user_uid(user_id) — prefix of pk_daily_reward_user(user_id, day_index)
DROP INDEX IF EXISTS idx_daily_reward_user_uid;
