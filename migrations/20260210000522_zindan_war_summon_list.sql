-- Zindan War monster summon list — spawn definitions per stage.
-- Source: MSSQL ZINDAN_WAR_SUMMON_LIST (0 rows, schema only)

CREATE TABLE IF NOT EXISTS zindan_war_summon_list (
    idx         INTEGER     NOT NULL,
    summon_type SMALLINT    NOT NULL,
    stage       SMALLINT    NOT NULL,
    sid         SMALLINT    NOT NULL,
    sid_count   SMALLINT    NOT NULL DEFAULT 1,
    pos_x       SMALLINT    NOT NULL DEFAULT 0,
    pos_z       SMALLINT    NOT NULL DEFAULT 0,
    range       SMALLINT    NOT NULL DEFAULT 0,
    summon_name VARCHAR(50),
    PRIMARY KEY (idx)
);

CREATE INDEX IF NOT EXISTS idx_zindan_summon_stage
    ON zindan_war_summon_list (stage);
