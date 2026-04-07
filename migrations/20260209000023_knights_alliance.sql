-- KNIGHTS_ALLIANCE: Alliance table for clan alliances
-- Source: MSSQL dbo.KNIGHTS_ALLIANCE
-- C++ Reference: _KNIGHTS_ALLIANCE in GameDefine.h
CREATE TABLE IF NOT EXISTS knights_alliance (
    s_main_alliance_knights  SMALLINT     NOT NULL PRIMARY KEY,
    s_sub_alliance_knights   SMALLINT     NOT NULL DEFAULT 0,
    s_mercenary_clan_1       SMALLINT     NOT NULL DEFAULT 0,
    s_mercenary_clan_2       SMALLINT     NOT NULL DEFAULT 0,
    str_alliance_notice      VARCHAR(128) NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_alliance_sub ON knights_alliance (s_sub_alliance_knights);
CREATE INDEX IF NOT EXISTS idx_alliance_merc1 ON knights_alliance (s_mercenary_clan_1);
CREATE INDEX IF NOT EXISTS idx_alliance_merc2 ON knights_alliance (s_mercenary_clan_2);
