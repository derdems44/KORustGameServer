-- Timed Notice table — periodic server announcements.
--
-- C++ Reference: TIMED_NOTICE table in MSSQL
-- Columns: nIndex (PK), noticetype (chat type), notice (text),
--          zoneid (0=all zones), time (interval in minutes).

CREATE TABLE IF NOT EXISTS timed_notice (
    n_index       INTEGER PRIMARY KEY,
    notice_type   SMALLINT NOT NULL DEFAULT 7,
    notice        TEXT NOT NULL DEFAULT '',
    zone_id       SMALLINT NOT NULL DEFAULT 0,
    time_minutes  INTEGER NOT NULL DEFAULT 1
);
