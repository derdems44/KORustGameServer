-- FerihaLog Audit Logging — unified audit log table
-- C++ Reference: FerihaLogHandler.cpp — 25 log functions, 24 separate MSSQL tables
-- Rust: Single table with event_type enum + JSONB details for flexibility

CREATE TABLE IF NOT EXISTS game_audit_log (
    id              BIGSERIAL PRIMARY KEY,
    event_type      SMALLINT    NOT NULL,
    account_name    VARCHAR(50) NOT NULL DEFAULT '',
    char_name       VARCHAR(50) NOT NULL DEFAULT '',
    remote_ip       VARCHAR(45) NOT NULL DEFAULT '',
    zone_id         SMALLINT    NOT NULL DEFAULT 0,
    pos_x           SMALLINT    NOT NULL DEFAULT 0,
    pos_z           SMALLINT    NOT NULL DEFAULT 0,
    details         TEXT        NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_audit_log_event_type   ON game_audit_log (event_type);
CREATE INDEX IF NOT EXISTS idx_audit_log_account      ON game_audit_log (account_name);
CREATE INDEX IF NOT EXISTS idx_audit_log_char         ON game_audit_log (char_name);
CREATE INDEX IF NOT EXISTS idx_audit_log_created      ON game_audit_log (created_at);
CREATE INDEX IF NOT EXISTS idx_audit_log_type_date    ON game_audit_log (event_type, created_at);

-- Partition-ready: created_at column allows easy future range partitioning
-- for high-volume log tables (e.g., monthly partitions).
