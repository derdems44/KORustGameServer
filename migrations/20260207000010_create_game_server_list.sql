-- GAME_SERVER_LIST: Sunucu listesi — Login Server tarafından istemciye gönderilir
-- Kaynak: MSSQL dbo.GAME_SERVER_LIST (12 kolon)
CREATE TABLE IF NOT EXISTS game_server_list (
    server_id       SMALLINT    NOT NULL PRIMARY KEY,
    group_id        SMALLINT    NOT NULL,
    screen_type     SMALLINT    NOT NULL DEFAULT 0,
    server_name     VARCHAR(40) NOT NULL,
    server_ip       VARCHAR(30) NOT NULL,
    lan_ip          VARCHAR(30) NOT NULL,
    player_cap      SMALLINT    NOT NULL DEFAULT 5000,
    free_player_cap SMALLINT    NOT NULL DEFAULT 4000,
    karus_king      VARCHAR(21) DEFAULT '',
    karus_notice    VARCHAR(100) DEFAULT '',
    elmorad_king    VARCHAR(21) DEFAULT '',
    elmorad_notice  VARCHAR(100) DEFAULT ''
);

-- Seed data from MSSQL reference (2 servers)
INSERT INTO game_server_list (server_id, group_id, screen_type, server_name, server_ip, lan_ip, player_cap, free_player_cap)
VALUES (1, 1, 3, 'PvpKo|PvpKo I', '127.0.0.1', '127.0.0.1', 5000, 4000)
ON CONFLICT (server_id) DO NOTHING;
