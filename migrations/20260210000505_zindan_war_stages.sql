-- Zindan War stage definitions
-- Source: MSSQL ZINDAN_WAR_STAGES (4 rows)
CREATE TABLE IF NOT EXISTS zindan_war_stages (
    idx         INTEGER PRIMARY KEY,
    stage_type  SMALLINT NOT NULL DEFAULT 0,
    stage       SMALLINT NOT NULL,
    time_min    SMALLINT NOT NULL          -- duration in minutes
);

INSERT INTO zindan_war_stages (idx, stage_type, stage, time_min) VALUES
(1, 0, 1, 5),
(2, 0, 2, 10),
(3, 0, 3, 15),
(4, 0, 4, 50)
ON CONFLICT (idx) DO NOTHING;
