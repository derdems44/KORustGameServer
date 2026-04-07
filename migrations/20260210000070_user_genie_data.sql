-- User Genie Data persistence table
-- Migrated from MSSQL USER_GENIE_DATA
-- Stores genie options blob, remaining time, and first-use flag per user account.
-- C++ Reference: CDBAgent::LoadGenieData / CDBAgent::UpdateGenieData

CREATE TABLE IF NOT EXISTS user_genie_data (
    user_id VARCHAR(21) NOT NULL PRIMARY KEY,
    genie_time INTEGER NOT NULL DEFAULT 0,
    genie_options BYTEA NOT NULL DEFAULT '\x00',
    first_using_genie SMALLINT NOT NULL DEFAULT 0
);
