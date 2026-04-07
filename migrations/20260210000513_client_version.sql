-- Client version tracking for patch system.
-- Source: MSSQL VERSION (4 rows)

CREATE TABLE IF NOT EXISTS client_version (
    version         SMALLINT    NOT NULL,
    history_version SMALLINT    NOT NULL,
    filename        VARCHAR(40) NOT NULL,
    PRIMARY KEY (version)
);

INSERT INTO client_version (version, history_version, filename) VALUES
    (2522, 2522, '2522.zip'),
    (2523, 2523, '2523.zip'),
    (2524, 2524, '2524.zip'),
    (2525, 2525, '2525.zip')
ON CONFLICT DO NOTHING;
