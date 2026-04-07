-- CURRENTUSER: Aktif oturum takibi
-- Kaynak: MSSQL dbo.CURRENTUSER (PK yok, strAccountID unique olmalı)
CREATE TABLE IF NOT EXISTS currentuser (
    str_account_id  VARCHAR(50)     NOT NULL PRIMARY KEY,
    str_char_id     VARCHAR(50)     NOT NULL,
    n_server_no     SMALLINT        NOT NULL,
    str_server_ip   VARCHAR(50)     NOT NULL,
    str_client_ip   VARCHAR(50)     NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_currentuser_char ON currentuser (str_char_id);
CREATE INDEX IF NOT EXISTS idx_currentuser_server ON currentuser (n_server_no);
