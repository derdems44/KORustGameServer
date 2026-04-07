-- Scheduled event start times
-- Source: MSSQL EVENT_START_SCHEDULE (18 rows, denormalized 5 time slots)
-- Normalized: event_start_schedule (event header) + event_start_time_slot (individual slots)
CREATE TABLE IF NOT EXISTS event_start_schedule (
    event_local_id  INTEGER PRIMARY KEY,
    event_type      SMALLINT NOT NULL DEFAULT 0,
    event_zone_id   SMALLINT NOT NULL,
    event_name      VARCHAR(50) NOT NULL,
    start_days      VARCHAR(20) NOT NULL DEFAULT '99',
    event_status    SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS event_start_time_slot (
    event_local_id  INTEGER NOT NULL REFERENCES event_start_schedule(event_local_id),
    slot_index      SMALLINT NOT NULL CHECK (slot_index BETWEEN 1 AND 5),
    start_hour      INTEGER NOT NULL DEFAULT 99,
    start_minute    INTEGER NOT NULL DEFAULT 99,
    time_active     SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (event_local_id, slot_index)
);

INSERT INTO event_start_schedule (event_local_id, event_type, event_zone_id, event_name, start_days, event_status) VALUES
(1,  1, 30, 'Castle SiegeWar',           '99', 0),
(2,  1, 61, 'Napies Gorge',              '99', 0),
(3,  1, 62, 'Alseids Prairie',           '99', 0),
(4,  1, 63, 'Nieds Triangle',            '99', 0),
(5,  1, 64, 'Nereids Island',            '99', 0),
(6,  1, 65, 'Zipang(War)',               '99', 0),
(7,  1, 66, 'Oreads(War)',               '99', 0),
(8,  1, 67, 'Test Zone',                 '99', 0),
(9,  1, 68, 'Snow War',                  '99', 0),
(10, 2, 84, 'Border Defance War',        '99', 0),
(11, 2, 85, 'Chaos Dengueon',            '99', 0),
(12, 2, 87, 'Juraid Mountain',           '99', 0),
(13, 2, 76, 'Knight Royale',             '99', 0),
(14, 3, 86, 'Under The Castle',          '99', 0),
(15, 3, 55, 'Forgetten Temple (46 - 59)','99', 0),
(16, 3, 55, 'Forgetten Temple (60 - 83)','99', 0),
(17, 3, 31, 'Beef Event',                '99', 0),
(18, 3, 91, 'ZindanWar',                 '99', 0)
ON CONFLICT (event_local_id) DO NOTHING;

-- Insert notable non-default time slots (Nereids slot2=19:30, BDW slot2/3/4, Chaos slot2/3/4, Juraid slot2/3/4, Beef slot2)
INSERT INTO event_start_time_slot (event_local_id, slot_index, start_hour, start_minute, time_active) VALUES
(5,  2, 19, 30, 0),
(10, 2, 99, 99, 99),
(10, 3, 99, 0,  99),
(10, 4, 99, 99, 99),
(11, 2, 99, 99, 99),
(11, 3, 99, 0,  99),
(12, 2, 99, 99, 99),
(12, 3, 99, 0,  99),
(17, 2, 99, 0,  0)
ON CONFLICT (event_local_id, slot_index) DO NOTHING;
