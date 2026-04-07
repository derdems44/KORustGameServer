-- Pet talk message templates.
-- Source: MSSQL PET_TALK (1 row)

CREATE TABLE IF NOT EXISTS pet_talk (
    idx     SMALLINT     NOT NULL,
    word    VARCHAR(50)  NOT NULL DEFAULT '',
    message VARCHAR(255) NOT NULL DEFAULT '',
    emo     VARCHAR(100) NOT NULL DEFAULT '',
    rand    INTEGER      NOT NULL DEFAULT 10000,
    PRIMARY KEY (idx)
);

INSERT INTO pet_talk (idx, word, message, emo, rand)
VALUES (1, 'test', 'test', 'npcimg\emoticon_base.dxt', 10000)
ON CONFLICT DO NOTHING;
