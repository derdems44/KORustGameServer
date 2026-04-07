-- Castle Siege War options table
-- Source: MSSQL KNIGHTS_CSW_OPT (1 row)
-- C++ Reference: CGameServerDlg CSW configuration

CREATE TABLE IF NOT EXISTS knights_csw_opt (
    id              SERIAL      PRIMARY KEY,
    preparing       SMALLINT    NOT NULL DEFAULT 10,
    war_time        SMALLINT    NOT NULL DEFAULT 60,
    money           INTEGER     NOT NULL DEFAULT 0,
    tl              INTEGER     NOT NULL DEFAULT 0,
    cash            INTEGER     NOT NULL DEFAULT 0,
    loyalty         INTEGER     NOT NULL DEFAULT 0,
    item_id_1       INTEGER     NOT NULL DEFAULT 0,
    item_count_1    INTEGER     NOT NULL DEFAULT 0,
    item_time_1     INTEGER     NOT NULL DEFAULT 0,
    item_id_2       INTEGER     NOT NULL DEFAULT 0,
    item_count_2    INTEGER     NOT NULL DEFAULT 0,
    item_time_2     INTEGER     NOT NULL DEFAULT 0,
    item_id_3       INTEGER     NOT NULL DEFAULT 0,
    item_count_3    INTEGER     NOT NULL DEFAULT 0,
    item_time_3     INTEGER     NOT NULL DEFAULT 0
);

INSERT INTO knights_csw_opt
    (preparing, war_time, money, tl, cash, loyalty,
     item_id_1, item_count_1, item_time_1,
     item_id_2, item_count_2, item_time_2,
     item_id_3, item_count_3, item_time_3)
VALUES
    (10, 60, 0, 0, 2000, 0,
     0, 0, 0,
     0, 0, 0,
     0, 0, 0)
ON CONFLICT (id) DO NOTHING;
