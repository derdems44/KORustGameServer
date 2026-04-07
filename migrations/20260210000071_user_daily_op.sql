-- User Daily Operations tracking table
-- Migrated from MSSQL USER_DAILY_OP
-- Tracks daily cooldowns / timestamps for various activities per user account.
-- Values are Unix timestamps (int); -1 means not yet used.

CREATE TABLE IF NOT EXISTS user_daily_op (
    user_id VARCHAR(21) NOT NULL PRIMARY KEY,
    chaos_map_time INTEGER NOT NULL DEFAULT -1,
    user_rank_reward_time INTEGER NOT NULL DEFAULT -1,
    personal_rank_reward_time INTEGER NOT NULL DEFAULT -1,
    king_wing_time INTEGER NOT NULL DEFAULT -1,
    warder_killer_time1 INTEGER NOT NULL DEFAULT 0,
    warder_killer_time2 INTEGER NOT NULL DEFAULT 0,
    keeper_killer_time INTEGER NOT NULL DEFAULT 0,
    user_loyalty_wing_reward_time INTEGER NOT NULL DEFAULT 0,
    full_moon_rift_map_time INTEGER NOT NULL DEFAULT -1,
    copy_information_time INTEGER NOT NULL DEFAULT -1
);
