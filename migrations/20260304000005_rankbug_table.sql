-- RANKBUG configuration table for ranking system multipliers.
-- C++ Reference: GameDefine.h:4898-4907 — struct RANKBUG
-- Loaded via ClickRankBugSet.h from MSSQL RANKBUG table.
CREATE TABLE IF NOT EXISTS rankbug (
    id              SERIAL PRIMARY KEY,
    border_join     INTEGER NOT NULL DEFAULT 0,
    chaos_join      INTEGER NOT NULL DEFAULT 0,
    juraid_join     INTEGER NOT NULL DEFAULT 0,
    cz_rank         INTEGER NOT NULL DEFAULT 0,
    cr_min_comp     INTEGER NOT NULL DEFAULT 0,
    cr_max_comp     INTEGER NOT NULL DEFAULT 0,
    lottery_join    INTEGER NOT NULL DEFAULT 0
);

-- Insert default row (all zeros — matches C++ RANKBUG::Initialize())
INSERT INTO rankbug (border_join, chaos_join, juraid_join, cz_rank, cr_min_comp, cr_max_comp, lottery_join)
VALUES (0, 0, 0, 0, 0, 0, 0)
ON CONFLICT DO NOTHING;
