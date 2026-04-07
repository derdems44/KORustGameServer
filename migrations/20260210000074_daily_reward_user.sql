-- Daily Reward User tracking table
-- Migrated from MSSQL DAILY_REWARD_USER
-- Tracks which daily rewards a user has claimed (up to 25 day slots).
-- The original MSSQL stores this as a 50-byte binary blob of (claimed, day_index) pairs.
-- We normalize this into a proper relational structure.

CREATE TABLE IF NOT EXISTS daily_reward_user (
    user_id VARCHAR(27) NOT NULL,
    day_index SMALLINT NOT NULL,
    claimed BOOLEAN NOT NULL DEFAULT false,
    CONSTRAINT pk_daily_reward_user PRIMARY KEY (user_id, day_index)
);

CREATE INDEX IF NOT EXISTS idx_daily_reward_user_uid ON daily_reward_user(user_id);
