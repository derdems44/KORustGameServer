-- Migration 083: ITEMUP_PROBABILITY table
-- MSSQL source: ITEMUP_PROBABILITY (1 row)
-- C++ Reference: CGameServerDlg::m_ItemUpProbability
--
-- Controls item upgrade probability modifiers.
-- bType: probability configuration type
-- nMaxSuccess/nMaxFail: maximum success/fail streak counters
-- nCurSuccess/nCurFail: current success/fail streak counters

CREATE TABLE IF NOT EXISTS itemup_probability (
    b_type      SMALLINT    NOT NULL,
    max_success SMALLINT    NOT NULL DEFAULT 1,
    max_fail    SMALLINT    NOT NULL DEFAULT 1,
    cur_success SMALLINT    NOT NULL DEFAULT 1,
    cur_fail    SMALLINT    NOT NULL DEFAULT 1,
    PRIMARY KEY (b_type)
);

-- Seed data from MSSQL (1 row)
INSERT INTO itemup_probability (b_type, max_success, max_fail, cur_success, cur_fail)
VALUES (1, 1, 1, 1, 1)
ON CONFLICT (b_type) DO NOTHING;
