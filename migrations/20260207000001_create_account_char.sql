-- ACCOUNT_CHAR: Hesap-karakter eşleştirme tablosu
-- Kaynak: MSSQL dbo.ACCOUNT_CHAR (PK: strAccountID)
-- Not: CHAR(N) → VARCHAR(N) trailing space prevention için
CREATE TABLE IF NOT EXISTS account_char (
    str_account_id  VARCHAR(21)     NOT NULL PRIMARY KEY,
    b_nation        SMALLINT        NOT NULL,
    b_char_num      SMALLINT        NOT NULL DEFAULT 0,
    str_char_id1    VARCHAR(21),
    str_char_id2    VARCHAR(21),
    str_char_id3    VARCHAR(21),
    str_char_id4    VARCHAR(21)
);

CREATE INDEX IF NOT EXISTS idx_account_char_nation ON account_char (b_nation);
