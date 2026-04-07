-- Banishment winner monster spawn positions (Bifrost/CSW event rewards)
-- Source: MSSQL BANISH_OF_WINNER (10 rows)
CREATE TABLE IF NOT EXISTS banish_of_winner (
    idx         INTEGER PRIMARY KEY,
    sid         SMALLINT NOT NULL,       -- monster SID
    nation_id   SMALLINT,                -- 1=Karus, 2=ElMorad
    zone_id     SMALLINT NOT NULL,
    pos_x       SMALLINT NOT NULL,
    pos_z       SMALLINT NOT NULL,
    spawn_count SMALLINT NOT NULL,
    radius      SMALLINT,
    dead_time   SMALLINT NOT NULL         -- respawn seconds
);

INSERT INTO banish_of_winner (idx, sid, nation_id, zone_id, pos_x, pos_z, spawn_count, radius, dead_time) VALUES
(1,  3252, 2, 62, 99,  77,  5, 20, 10),
(2,  3252, 2, 62, 130, 76,  5, 15, 10),
(3,  3252, 2, 62, 123, 118, 5, 18, 10),
(4,  3252, 2, 62, 144, 136, 5, 25, 10),
(5,  3252, 2, 62, 106, 167, 5, 15, 10),
(6,  3202, 1, 62, 933, 931, 5, 20, 10),
(7,  3202, 1, 62, 901, 950, 5, 15, 10),
(8,  3202, 1, 62, 904, 905, 5, 19, 10),
(9,  3202, 1, 62, 918, 876, 5, 25, 10),
(10, 3202, 1, 62, 853, 927, 5, 13, 10)
ON CONFLICT (idx) DO NOTHING;
