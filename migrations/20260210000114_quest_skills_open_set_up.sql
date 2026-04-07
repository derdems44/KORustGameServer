-- Quest skill open setup conditions
-- MSSQL source: QUEST_SKILLS_OPEN_SET_UP (20 rows)

CREATE TABLE IF NOT EXISTS quest_skills_open_set_up (
    n_index             INT      NOT NULL PRIMARY KEY,
    n_event_data_index  SMALLINT NOT NULL
);

INSERT INTO quest_skills_open_set_up (n_index, n_event_data_index) VALUES
(1, 334),
(2, 335),
(3, 336),
(4, 337),
(5, 348),
(6, 349),
(7, 357),
(8, 359),
(9, 360),
(10, 361),
(11, 362),
(12, 363),
(13, 364),
(14, 365),
(15, 366),
(16, 367),
(17, 368),
(18, 1377),
(19, 1378),
(20, 347)
ON CONFLICT DO NOTHING;
