-- User Return Data table
-- Migrated from MSSQL USER_RETURN_DATA
-- Tracks returning user eligibility and symbol timing per character.

CREATE TABLE IF NOT EXISTS user_return_data (
    character_id VARCHAR(21) NOT NULL PRIMARY KEY,
    return_symbol_ok SMALLINT DEFAULT 0,
    return_logout_time BIGINT DEFAULT 0,
    return_symbol_time BIGINT DEFAULT 0
);
