-- TB_USER: Hesap (account) tablosu — login flow için kritik
-- Kaynak: MSSQL dbo.TB_USER (31 kolon)
-- Not: INT IDENTITY → BIGINT GENERATED ALWAYS AS IDENTITY
CREATE TABLE IF NOT EXISTS tb_user (
    id                  BIGINT          GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    str_account_id      VARCHAR(21)     NOT NULL UNIQUE,
    str_passwd          VARCHAR(28)     NOT NULL,
    str_seal_passwd     VARCHAR(8)      NOT NULL DEFAULT '12345678',
    otp_password        VARCHAR(6)      NOT NULL DEFAULT '123456',
    str_client_ip       VARCHAR(15),
    b_premium_type      SMALLINT        NOT NULL DEFAULT 0,
    dt_premium_time     TIMESTAMPTZ     DEFAULT NOW(),
    s_hours             SMALLINT        NOT NULL DEFAULT 0,
    dt_create_time      TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    cash_point          INTEGER         NOT NULL DEFAULT 0,
    email               VARCHAR(50),
    security_answer     VARCHAR(50),
    security_question   VARCHAR(50),
    str_authority       SMALLINT        NOT NULL DEFAULT 1,
    user_name           VARCHAR(50),
    user_surname        VARCHAR(50),
    bonus_cash_point    INTEGER         DEFAULT 0,
    user_phone_number   VARCHAR(10),
    punishment_date     TIMESTAMPTZ,
    punishment_period   INTEGER         DEFAULT 0,
    account_check       SMALLINT        NOT NULL DEFAULT 1,
    promotion_check     SMALLINT        NOT NULL DEFAULT 0,
    str_genie_time      INTEGER         NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_tb_user_authority ON tb_user (str_authority);
CREATE INDEX IF NOT EXISTS idx_tb_user_premium ON tb_user (b_premium_type);
