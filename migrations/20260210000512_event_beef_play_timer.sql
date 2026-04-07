-- BDW (Border Defence War) event timer configuration.
-- Source: MSSQL EVENT_BEEF_PLAY_TIMER (1 row)

CREATE TABLE IF NOT EXISTS event_beef_play_timer (
    event_local_id   SMALLINT    NOT NULL,
    event_zone_id    SMALLINT    NOT NULL,
    event_name       VARCHAR(50) NOT NULL,
    monument_time    INTEGER     NOT NULL DEFAULT 1,
    loser_sign_time  INTEGER     NOT NULL DEFAULT 1,
    farming_time     INTEGER     NOT NULL DEFAULT 30,
    PRIMARY KEY (event_local_id)
);

INSERT INTO event_beef_play_timer (event_local_id, event_zone_id, event_name, monument_time, loser_sign_time, farming_time)
VALUES (14, 31, 'Beef Event', 1, 1, 30)
ON CONFLICT DO NOTHING;
