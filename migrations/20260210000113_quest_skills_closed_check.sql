-- Quest skill closed check prerequisites
-- MSSQL source: QUEST_SKILLS_CLOSED_CHECK (25 rows)

CREATE TABLE IF NOT EXISTS quest_skills_closed_check (
    n_index             INT      NOT NULL PRIMARY KEY,
    s_event_data_index  SMALLINT NOT NULL,
    n_nation            SMALLINT
);

INSERT INTO quest_skills_closed_check (n_index, s_event_data_index, n_nation) VALUES
(1, 500, 1),
(2, 334, 1),
(3, 335, 1),
(4, 336, 1),
(5, 337, 1),
(6, 347, 1),
(7, 348, 1),
(8, 349, 1),
(9, 357, 1),
(10, 359, 1),
(11, 360, 1),
(12, 361, 1),
(13, 362, 1),
(14, 363, 1),
(15, 364, 1),
(16, 365, 1),
(17, 366, 1),
(18, 367, 1),
(19, 368, 1),
(20, 1377, 1),
(21, 1378, 1),
(22, 1119, 3),
(23, 1120, 3),
(24, 1121, 3),
(25, 1122, 3)
ON CONFLICT DO NOTHING;
