-- Schema Cleanup: Drop redundant indexes
--
-- All 5 tables that were flagged for missing PKs already have primary keys:
--   event_opt_ft, ft_summon_list, item_op, item_sell_table, item_upgrade_settings
--
-- This migration removes 7 redundant indexes:
--   1. idx_new_upgrade_origin          — exact duplicate of idx_new_upgrade_origin_number
--   2. idx_item_sell_table_nindex      — duplicate of item_sell_table_pkey (n_index)
--   3. idx_item_op_item_id             — duplicate of item_op_pkey (item_id)
--   4. idx_account_premium_account     — prefix of account_premium_pkey (account_id, slot)
--   5. idx_friend_list_user            — prefix of friend_list_pkey (user_id, friend_name)
--   6. idx_quest_skills_closed_data_user — duplicate of quest_skills_closed_data_pkey (str_user_id)
--   7. idx_challenge_summon_level      — prefix of idx_challenge_summon_stage (b_level, b_stage)

-- 1) Exact duplicate on new_upgrade.origin_number (two identical indexes)
DROP INDEX IF EXISTS idx_new_upgrade_origin;

-- 2) Redundant: idx_item_sell_table_nindex duplicates item_sell_table_pkey
DROP INDEX IF EXISTS idx_item_sell_table_nindex;

-- 3) Redundant: idx_item_op_item_id duplicates item_op_pkey
DROP INDEX IF EXISTS idx_item_op_item_id;

-- 4) Redundant: idx_account_premium_account(account_id) is a prefix of PK(account_id, slot)
DROP INDEX IF EXISTS idx_account_premium_account;

-- 5) Redundant: idx_friend_list_user(user_id) is a prefix of PK(user_id, friend_name)
DROP INDEX IF EXISTS idx_friend_list_user;

-- 6) Redundant: idx_quest_skills_closed_data_user(str_user_id) duplicates PK(str_user_id)
DROP INDEX IF EXISTS idx_quest_skills_closed_data_user;

-- 7) Redundant: idx_challenge_summon_level(b_level) is a prefix of idx_challenge_summon_stage(b_level, b_stage)
DROP INDEX IF EXISTS idx_challenge_summon_level;
