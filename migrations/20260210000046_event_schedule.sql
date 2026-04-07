-- Event scheduling tables (migrated from MSSQL EVENT_* tables).
-- Used by the automatic event timer system to schedule BDW, Chaos, Juraid, FT, etc.
--
-- C++ Reference: GameServerDlg.h — pEventTimeOpt, m_EventTimerShowListArray, etc.

-- EVENT_SCHEDULE_MAIN_LIST: Core schedule configuration per event.
-- Each row defines an event's type, zone, up to 5 start times, and requirements.
-- C++ Reference: CEventScheduleMainListSet
CREATE TABLE IF NOT EXISTS event_schedule_main_list (
    eventid         SMALLINT    NOT NULL PRIMARY KEY,
    event_type      SMALLINT    NOT NULL DEFAULT 0,   -- 1=LunarWar, 2=VirtualRoom, 3=SingleRoom
    zoneid          SMALLINT    NOT NULL DEFAULT 0,
    name            VARCHAR(50) NOT NULL DEFAULT '',
    status          SMALLINT    NOT NULL DEFAULT 0,    -- 0=disabled, 1=enabled
    hour1           SMALLINT    NOT NULL DEFAULT -1,
    minute1         SMALLINT    NOT NULL DEFAULT -1,
    hour2           SMALLINT    NOT NULL DEFAULT -1,
    minute2         SMALLINT    NOT NULL DEFAULT -1,
    hour3           SMALLINT    NOT NULL DEFAULT -1,
    minute3         SMALLINT    NOT NULL DEFAULT -1,
    hour4           SMALLINT    NOT NULL DEFAULT -1,
    minute4         SMALLINT    NOT NULL DEFAULT -1,
    hour5           SMALLINT    NOT NULL DEFAULT -1,
    minute5         SMALLINT    NOT NULL DEFAULT -1,
    min_level       SMALLINT    NOT NULL DEFAULT 0,
    max_level       SMALLINT    NOT NULL DEFAULT 0,
    req_loyalty     INT         NOT NULL DEFAULT 0,
    req_money       INT         NOT NULL DEFAULT 0
);

-- EVENT_SCHEDULE_DAY_LIST: Per-event day-of-week enablement (0=Sun..6=Sat).
-- C++ Reference: CEventScheduleDayListSet
CREATE TABLE IF NOT EXISTS event_schedule_day_list (
    eventid     SMALLINT NOT NULL PRIMARY KEY,
    sunday      SMALLINT NOT NULL DEFAULT 0,
    monday      SMALLINT NOT NULL DEFAULT 0,
    tuesday     SMALLINT NOT NULL DEFAULT 0,
    wednesday   SMALLINT NOT NULL DEFAULT 0,
    thursday    SMALLINT NOT NULL DEFAULT 0,
    friday      SMALLINT NOT NULL DEFAULT 0,
    saturday    SMALLINT NOT NULL DEFAULT 0
);

-- EVENT_OPT_VROOM: Virtual-room event timing options (sign-up, play, attack windows).
-- Keyed by zoneid — one row per room-based event type (BDW=84, Chaos=85, JR=87).
-- C++ Reference: CGameServerDlg::pEventTimeOpt.pvroomop[]
CREATE TABLE IF NOT EXISTS event_opt_vroom (
    zoneid      SMALLINT    NOT NULL PRIMARY KEY,
    name        VARCHAR(30) NOT NULL DEFAULT '',
    sign        INT         NOT NULL DEFAULT 0,    -- sign-up time in minutes
    play        INT         NOT NULL DEFAULT 0,    -- play time in minutes
    attackopen  INT         NOT NULL DEFAULT 0,    -- minutes after sign for attack open
    attackclose INT         NOT NULL DEFAULT 0,    -- minutes after sign for attack close
    finish      INT         NOT NULL DEFAULT 20    -- seconds after winner screen before cleanup
);

-- EVENT_ROOM_PLAY_TIMER: Room-based event detailed timing.
-- C++ Reference: eventroomscheduleplaytimerset.h
CREATE TABLE IF NOT EXISTS event_room_play_timer (
    event_local_id      SMALLINT    NOT NULL PRIMARY KEY,
    event_zone_id       SMALLINT    NOT NULL DEFAULT 0,
    event_name          VARCHAR(50) NOT NULL DEFAULT '',
    event_sign_time     INT         NOT NULL DEFAULT 0,   -- minutes
    event_play_time     INT         NOT NULL DEFAULT 0,   -- minutes
    event_attack_open   INT         NOT NULL DEFAULT 0,   -- minutes
    event_attack_close  INT         NOT NULL DEFAULT 0,   -- minutes
    event_finish_time   INT         NOT NULL DEFAULT 0    -- minutes
);

-- EVENT_REWARDS: Winner/loser rewards for room-based events.
-- C++ Reference: CGameServerDlg::m_EventRewardArray
CREATE TABLE IF NOT EXISTS event_rewards (
    s_index         SERIAL      PRIMARY KEY,
    status          BOOLEAN     NOT NULL DEFAULT true,
    local_id        SMALLINT    NOT NULL DEFAULT 0,      -- EventLocalID (9=BDW, 11=JR)
    is_winner       BOOLEAN     NOT NULL DEFAULT false,
    description     VARCHAR(50) NOT NULL DEFAULT '',
    item_id1        INT         NOT NULL DEFAULT 0,
    item_count1     INT         NOT NULL DEFAULT 0,
    item_expiration1 INT        NOT NULL DEFAULT 0,
    item_id2        INT         NOT NULL DEFAULT 0,
    item_count2     INT         NOT NULL DEFAULT 0,
    item_expiration2 INT        NOT NULL DEFAULT 0,
    item_id3        INT         NOT NULL DEFAULT 0,
    item_count3     INT         NOT NULL DEFAULT 0,
    item_expiration3 INT        NOT NULL DEFAULT 0,
    experience      BIGINT      NOT NULL DEFAULT 0,
    loyalty         INT         NOT NULL DEFAULT 0,
    cash            INT         NOT NULL DEFAULT 0,
    noah            INT         NOT NULL DEFAULT 0
);

-- EVENT_TRIGGER: Maps NPC types + IDs to trigger numbers (quest/event triggers).
-- C++ Reference: CGameServerDlg::m_EventTriggerArray
CREATE TABLE IF NOT EXISTS event_trigger (
    n_index      INT      NOT NULL PRIMARY KEY,
    b_npc_type   SMALLINT NOT NULL DEFAULT 0,
    s_npc_id     SMALLINT NOT NULL DEFAULT 0,
    n_trigger_num INT     NOT NULL DEFAULT 0
);

-- EVENT_TIMER_SHOW_LIST: Client UI timer display entries.
-- C++ Reference: CGameServerDlg::m_EventTimerShowListArray
CREATE TABLE IF NOT EXISTS event_timer_show_list (
    id          SERIAL      PRIMARY KEY,
    name        VARCHAR(50) NOT NULL DEFAULT '',
    status      BOOLEAN     NOT NULL DEFAULT false,
    hour        INT         NOT NULL DEFAULT 0,
    minute      INT         NOT NULL DEFAULT 0,
    days        VARCHAR(30) NOT NULL DEFAULT ''
);

-- EVENT_OPT_FT: Forgotten Temple specific timing options.
-- C++ Reference: CGameServerDlg::pForgettenTemple.ptimeopt
CREATE TABLE IF NOT EXISTS event_opt_ft (
    playing_time   INT NOT NULL DEFAULT 30,   -- minutes
    summon_time    INT NOT NULL DEFAULT 300,  -- seconds between summon waves
    spawn_min_time INT NOT NULL DEFAULT 10,   -- minimum seconds between spawns
    waiting_time   INT NOT NULL DEFAULT 20,   -- seconds to wait before starting
    min_level      INT NOT NULL DEFAULT 60,
    max_level      INT NOT NULL DEFAULT 83
);

-- Seed EVENT_SCHEDULE_MAIN_LIST (14 rows from MSSQL)
INSERT INTO event_schedule_main_list (eventid, event_type, zoneid, name, status, hour1, minute1, hour2, minute2, hour3, minute3, hour4, minute4, hour5, minute5, min_level, max_level, req_loyalty, req_money) VALUES
(1,  1, 30, 'Castle SiegeWar',    1, 22, 0,  -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(2,  1, 61, 'Napies Gorge',       0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(3,  1, 62, 'Alseids Prairie',    0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(4,  1, 63, 'Nieds Triangle',     0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(5,  1, 64, 'Nereids Island',     1, 12, 0,  19, 30, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(6,  1, 65, 'Zipang(War)',        0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(7,  1, 66, 'Oreads(War)',        0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(8,  1, 68, 'Snow War',           0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(9,  2, 84, 'Border Defance War', 1,  9, 0,  14, 0,  19, 0,   2, 0,   6, 0,  35, 83, 0, 0),
(10, 2, 85, 'Chaos Dengueon',     1, 11, 0,  16, 0,  21, 0,   1, 0,  -1, -1, 10, 83, 0, 0),
(11, 2, 87, 'Juraid Mountain',    1, 10, 0,  15, 0,  22, 0,   3, 0,  -1, -1, 60, 83, 0, 0),
(12, 3, 86, 'Under The Castle',   0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(13, 3, 55, 'Forgetten Temple',   0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,  0,  0, 0, 0),
(14, 3, 31, 'Beef Event',         1, 13, 0,  23, 0,  -1, -1, -1, -1, -1, -1, 65, 83, 0, 0)
ON CONFLICT (eventid) DO NOTHING;

-- Seed EVENT_SCHEDULE_DAY_LIST (14 rows from MSSQL)
INSERT INTO event_schedule_day_list (eventid, sunday, monday, tuesday, wednesday, thursday, friday, saturday) VALUES
(1,  1, 0, 0, 0, 0, 0, 0),
(2,  0, 0, 0, 0, 0, 0, 0),
(3,  0, 1, 0, 0, 0, 0, 1),
(4,  0, 0, 0, 0, 0, 0, 0),
(5,  0, 1, 0, 0, 0, 0, 1),
(6,  0, 0, 0, 0, 0, 0, 0),
(7,  0, 0, 0, 0, 0, 0, 0),
(8,  0, 0, 0, 0, 0, 0, 0),
(9,  1, 1, 1, 1, 1, 1, 1),
(10, 1, 1, 1, 1, 1, 1, 1),
(11, 1, 1, 1, 1, 1, 1, 1),
(12, 0, 0, 0, 0, 0, 0, 0),
(13, 1, 1, 1, 1, 1, 1, 1),
(14, 0, 0, 0, 0, 0, 0, 0)
ON CONFLICT (eventid) DO NOTHING;

-- Seed EVENT_OPT_VROOM (3 rows from MSSQL)
INSERT INTO event_opt_vroom (zoneid, name, sign, play, attackopen, attackclose, finish) VALUES
(84, 'BDW',   10, 15, 0, 30, 20),
(85, 'CHAOS', 10, 10, 0, 20, 20),
(87, 'JR',    10, 50, 0, 50, 20)
ON CONFLICT (zoneid) DO NOTHING;

-- Seed EVENT_ROOM_PLAY_TIMER (3 rows from MSSQL)
INSERT INTO event_room_play_timer (event_local_id, event_zone_id, event_name, event_sign_time, event_play_time, event_attack_open, event_attack_close, event_finish_time) VALUES
(10, 84, 'Border Defance War', 10, 30, 0, 30, 30),
(11, 85, 'Chaos Dengueon',     10, 20, 0, 20, 20),
(12, 87, 'Juraid Mountain',     1, 50, 0, 50, 50)
ON CONFLICT (event_local_id) DO NOTHING;

-- Seed EVENT_REWARDS (4 rows from MSSQL)
INSERT INTO event_rewards (status, local_id, is_winner, description, item_id1, item_count1, item_expiration1, item_id2, item_count2, item_expiration2, item_id3, item_count3, item_expiration3, experience, loyalty, cash, noah) VALUES
(true, 9,  true,  'bdw winner', 900017000, 1, 0, 0, 0, 0, 0, 0, 0, 200000000, 1000, 100, 10000000),
(true, 9,  false, 'bdw loser',  389301000, 1, 0, 0, 0, 0, 0, 0, 0, 50000000,  300,  10,  500000),
(true, 11, true,  'jr winner',  900017000, 1, 0, 0, 0, 0, 0, 0, 0, 250000000, 1000, 100, 10000000),
(true, 11, false, 'jr loser',   389301000, 1, 0, 0, 0, 0, 0, 0, 0, 50000000,  300,  10,  500000)
ON CONFLICT DO NOTHING;

-- Seed EVENT_OPT_FT (1 row from MSSQL)
INSERT INTO event_opt_ft (playing_time, summon_time, spawn_min_time, waiting_time, min_level, max_level) VALUES
(30, 300, 10, 20, 60, 83)
ON CONFLICT DO NOTHING;
