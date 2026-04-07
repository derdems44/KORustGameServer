-- Scheduled broadcast messages
-- Source: MSSQL SEND_MESSAGES (4 rows)
CREATE TABLE IF NOT EXISTS send_messages (
    id              SERIAL PRIMARY KEY,
    message         VARCHAR(250),
    sender          VARCHAR(21) NOT NULL,
    chat_type       SMALLINT NOT NULL DEFAULT 2,
    send_type       SMALLINT NOT NULL DEFAULT 1,
    send_hour_minute SMALLINT NOT NULL DEFAULT 0
);

-- Automatic scheduled GM commands
-- Source: MSSQL AUTOMATIC_COMMAND (3 rows)
CREATE TABLE IF NOT EXISTS automatic_command (
    idx         SERIAL PRIMARY KEY,
    status      BOOLEAN NOT NULL DEFAULT true,
    command     VARCHAR(300) NOT NULL DEFAULT '-',
    hour        INTEGER NOT NULL DEFAULT 0,
    minute      INTEGER NOT NULL DEFAULT 0,
    day_of_week INTEGER NOT NULL DEFAULT 0,   -- 0=Sun..6=Sat, 7=every day
    description VARCHAR(50) NOT NULL DEFAULT '-'
);

INSERT INTO automatic_command (idx, status, command, hour, minute, day_of_week, description) VALUES
(1, true, '/beefclose', 10, 0, 7, 'Beef Close'),
(2, true, '/beefclose', 16, 0, 7, 'Beef Close'),
(3, true, '/beefclose', 22, 0, 7, 'Beef Close')
ON CONFLICT (idx) DO NOTHING;
