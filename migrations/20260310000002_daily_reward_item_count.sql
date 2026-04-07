-- Add item_count column to daily_reward for configurable reward quantities.
-- Binary/ HandleDailyRewardGive uses a per-day count array; our table only had item_id.
-- Also add reset_month to daily_reward_user for monthly progress reset detection.

ALTER TABLE daily_reward ADD COLUMN IF NOT EXISTS item_count SMALLINT NOT NULL DEFAULT 1;

ALTER TABLE daily_reward_user ADD COLUMN IF NOT EXISTS last_claim_month SMALLINT NOT NULL DEFAULT 0;
