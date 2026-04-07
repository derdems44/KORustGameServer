-- Game server options / configuration
-- Source: MSSQL GAME_OPTIONS (1 row)
CREATE TABLE IF NOT EXISTS game_options (
    id              INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),  -- singleton row
    maintenance_mode BOOLEAN NOT NULL DEFAULT false,
    char_select_login BOOLEAN NOT NULL DEFAULT false,
    open_otp        BOOLEAN NOT NULL DEFAULT false,
    auto_register   BOOLEAN NOT NULL DEFAULT true,
    free_limit      SMALLINT NOT NULL DEFAULT 4000,
    total_user_limit SMALLINT NOT NULL DEFAULT 9000,
    server_ip       VARCHAR(15) NOT NULL DEFAULT '127.0.0.1'
);

INSERT INTO game_options (id, maintenance_mode, char_select_login, open_otp, auto_register, free_limit, total_user_limit, server_ip)
VALUES (1, false, false, false, true, 4000, 9000, '127.0.0.1')
ON CONFLICT (id) DO NOTHING;
