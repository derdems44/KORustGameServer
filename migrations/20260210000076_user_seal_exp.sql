-- User Seal Experience table
-- Migrated from MSSQL USER_SEAL_EXP
-- Tracks sealed (banked) experience per user account.

CREATE TABLE IF NOT EXISTS user_seal_exp (
    user_id VARCHAR(21) NOT NULL PRIMARY KEY,
    sealed_exp INTEGER NOT NULL DEFAULT 0
);
