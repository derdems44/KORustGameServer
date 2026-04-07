-- Account ban/check records
-- Source: MSSQL CHECK_ACCOUNT (150 rows, runtime data - schema only)
CREATE TABLE IF NOT EXISTS check_account (
    account_id      VARCHAR(21) NOT NULL PRIMARY KEY,
    gm              VARCHAR(21) NOT NULL DEFAULT '-',
    login_time_status INTEGER NOT NULL DEFAULT 1,
    reason          VARCHAR(250) NOT NULL DEFAULT '-',
    ban_count       INTEGER NOT NULL DEFAULT 0,
    open_count      INTEGER NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_check_account_status ON check_account(login_time_status);
