-- LEVEL_UP: Seviye-tecrübe tablosu
-- Kaynak: MSSQL dbo.LEVEL_UP (PK: ID)
CREATE TABLE IF NOT EXISTS level_up (
    id              SMALLINT    NOT NULL PRIMARY KEY,
    level           SMALLINT    NOT NULL,
    exp             BIGINT      NOT NULL,
    rebirth_level   SMALLINT    DEFAULT 0
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_level_up_level ON level_up (level, rebirth_level);
