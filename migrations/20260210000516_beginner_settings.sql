-- Beginner/new player protection settings.
-- Source: MSSQL BEGINNER_SETTINGS (1 row)
-- beginner_type: 0=False, 1=Ardream, 2=Ronark Land Base, 3=Ronark Land, 4=62Level

CREATE TABLE IF NOT EXISTS beginner_settings (
    server_no      SMALLINT     NOT NULL,
    beginner_type  SMALLINT     NOT NULL DEFAULT 0,
    description    VARCHAR(255),
    PRIMARY KEY (server_no)
);

INSERT INTO beginner_settings (server_no, beginner_type, description)
VALUES (1, 0, '1:Ardream || 2:Ronark Land Base || 3:Ronark Land || 4:62Level || 0:False')
ON CONFLICT DO NOTHING;
