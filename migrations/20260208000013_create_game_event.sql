-- Game events — zone change triggers, traps, and area effects.
-- Source: MSSQL EVENT table (103 events)
--
-- event_type: 1 = ZONE_CHANGE, 2 = TRAP_DEAD, 3 = TRAP_AREA
-- exec1/2/3: for ZONE_CHANGE → dest_zone, dest_x, dest_z

CREATE TABLE IF NOT EXISTS game_event (
    zone_no    SMALLINT NOT NULL,
    event_num  SMALLINT NOT NULL,
    event_type SMALLINT NOT NULL,
    cond1      INTEGER NOT NULL DEFAULT 0,
    cond2      INTEGER NOT NULL DEFAULT 0,
    cond3      INTEGER NOT NULL DEFAULT 0,
    cond4      INTEGER NOT NULL DEFAULT 0,
    cond5      INTEGER NOT NULL DEFAULT 0,
    exec1      INTEGER NOT NULL DEFAULT 0,
    exec2      INTEGER NOT NULL DEFAULT 0,
    exec3      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (zone_no, event_num)
);

-- Seed data from MSSQL EVENT table
-- Zone 1: Luferson Castle
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1001, 1, 61, 837, 168);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1002, 1, 62, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1003, 1, 63, 165, 193);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1004, 1, 64, 810, 781);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1005, 1, 65, 76, 729);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1006, 1, 66, 202, 846);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1011, 1, 69, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1021, 1, 69, 143, 73);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (1, 1031, 1, 11, 486, 557);

-- Zone 2: El Morad Castle
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 1005, 1, 65, 244, 945);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 1022, 1, 69, 900, 900);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2001, 1, 61, 95, 886);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2002, 1, 62, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2003, 1, 63, 829, 810);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2004, 1, 64, 234, 229);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2006, 1, 66, 822, 175);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2011, 1, 69, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (2, 2031, 1, 12, 486, 557);

-- Zone 5: Luferson Castle II
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1001, 1, 61, 837, 168);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1002, 1, 62, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1003, 1, 63, 165, 193);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1004, 1, 64, 810, 781);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1005, 1, 65, 76, 729);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1006, 1, 66, 202, 846);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1011, 1, 69, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1021, 1, 69, 143, 73);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (5, 1031, 1, 11, 486, 557);

-- Zone 6: Luferson Castle III
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1001, 1, 61, 837, 168);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1002, 1, 62, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1003, 1, 63, 165, 193);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1004, 1, 64, 810, 781);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1005, 1, 65, 76, 729);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1006, 1, 66, 202, 846);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1011, 1, 69, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1021, 1, 69, 143, 73);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (6, 1031, 1, 11, 486, 557);

-- Zone 7: El Morad Castle II
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 1005, 1, 65, 244, 945);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 1022, 1, 69, 900, 900);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2001, 1, 61, 95, 886);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2002, 1, 62, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2003, 1, 63, 829, 810);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2004, 1, 64, 234, 229);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2006, 1, 66, 822, 175);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2011, 1, 69, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (7, 2031, 1, 12, 486, 557);

-- Zone 8: El Morad Castle III
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 1005, 1, 65, 244, 945);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 1022, 1, 69, 900, 900);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2001, 1, 61, 95, 886);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2002, 1, 62, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2003, 1, 63, 829, 810);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2004, 1, 64, 234, 229);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2006, 1, 66, 822, 175);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2011, 1, 69, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (8, 2031, 1, 12, 486, 557);

-- Zone 11: Karus K_Eslant
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (11, 1131, 1, 1, 1365, 1841);

-- Zone 12: Elmorad E_Eslant
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (12, 1231, 1, 2, 678, 194);

-- Zone 13: Karus K_Eslant II
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (13, 1131, 1, 5, 1365, 1841);

-- Zone 14: Karus K_Eslant III
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (14, 1131, 1, 6, 1365, 1841);

-- Zone 15: Elmorad E_Eslant II
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (15, 1231, 1, 7, 678, 194);

-- Zone 16: Elmorad E_Eslant III
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (16, 1231, 1, 8, 678, 194);

-- Zone 18: Karus2004 (alternate)
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1001, 1, 61, 837, 168);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1002, 1, 62, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1003, 1, 63, 165, 193);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1004, 1, 64, 810, 781);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1005, 1, 65, 165, 135);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1006, 1, 66, 218, 885);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1011, 1, 69, 160, 67);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1021, 1, 69, 143, 73);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (18, 1031, 1, 11, 486, 557);

-- Zone 28: Elmo2004 (alternate)
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 1022, 1, 69, 900, 900);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2001, 1, 61, 95, 886);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2002, 1, 62, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2003, 1, 63, 829, 810);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2004, 1, 64, 234, 229);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2005, 1, 65, 160, 860);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2006, 1, 66, 805, 145);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2011, 1, 69, 864, 957);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2021, 1, 69, 900, 900);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (28, 2031, 1, 12, 486, 557);

-- Zone 31: Delos (TRAP_AREA)
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (31, 410, 3, 0, 0, 0);

-- Zone 61-66: Battle Zones
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (61, 1011, 1, 1, 219, 1859);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (61, 1012, 1, 2, 1859, 170);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (62, 1011, 1, 1, 219, 1859);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (62, 1012, 1, 2, 1859, 170);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (63, 1011, 1, 1, 219, 1859);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (63, 1012, 1, 2, 1859, 170);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (64, 1011, 1, 1, 219, 1895);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (64, 1012, 1, 2, 1895, 170);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (65, 1011, 1, 1, 219, 1895);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (65, 1012, 1, 2, 1895, 170);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (66, 1011, 1, 1, 219, 1895);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (66, 1012, 1, 2, 1895, 170);

-- Zone 69: Freezone
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (69, 1011, 1, 1, 912, 230);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (69, 1012, 1, 2, 1132, 1829);

-- Zone 71-73: Dungeons
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (71, 1011, 1, 1, 354, 1610);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (71, 1012, 1, 2, 1670, 370);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (72, 1011, 1, 1, 1933, 1708);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (72, 1012, 1, 2, 123, 1150);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (73, 1011, 1, 1, 1933, 1708);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (73, 1012, 1, 2, 123, 1150);

-- Zone 75-76: TRAP_AREA events
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (75, 500, 3, 0, 0, 0);
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (76, 410, 3, 0, 0, 0);

-- Zone 86: Special warp
INSERT INTO game_event (zone_no, event_num, event_type, exec1, exec2, exec3) VALUES (86, 2014, 1, 86, 838, 917);
