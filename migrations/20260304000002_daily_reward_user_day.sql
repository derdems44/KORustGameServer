-- Add day_of_month column to daily_reward_user for sequential claim validation.
-- C++ Reference: sGetDay[i] = strDateTime.GetDay() — stores day-of-month when claimed.
-- Used to prevent claiming two rewards on the same calendar day.
ALTER TABLE daily_reward_user ADD COLUMN IF NOT EXISTS day_of_month SMALLINT NOT NULL DEFAULT 0;
