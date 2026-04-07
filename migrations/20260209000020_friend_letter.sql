-- Migration #20: Friend list and letter (mail) tables
--
-- C++ Reference: FRIEND_LIST + MAIL_BOX tables in MSSQL

-- ── Friend List ─────────────────────────────────────────────────────────────
-- Normalized from C++ FRIEND_LIST (which stores 24 columns in a single row).
-- Max 24 friends per user enforced at application level.
CREATE TABLE IF NOT EXISTS friend_list (
    user_id     VARCHAR(21)  NOT NULL,
    friend_name VARCHAR(21)  NOT NULL,
    added_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, friend_name)
);

CREATE INDEX IF NOT EXISTS idx_friend_list_user ON friend_list (user_id);

-- ── Letter / Mail Box ───────────────────────────────────────────────────────
-- C++ Reference: MAIL_BOX table — stores in-game letters with optional item attachments.
CREATE TABLE IF NOT EXISTS letter (
    letter_id    SERIAL       PRIMARY KEY,
    sender_name  VARCHAR(21)  NOT NULL,
    recipient_name VARCHAR(21) NOT NULL,
    subject      VARCHAR(32)  NOT NULL DEFAULT '',
    message      VARCHAR(128) NOT NULL DEFAULT '',
    b_type       SMALLINT     NOT NULL DEFAULT 1,
    item_id      INTEGER      NOT NULL DEFAULT 0,
    item_count   SMALLINT     NOT NULL DEFAULT 0,
    item_durability SMALLINT  NOT NULL DEFAULT 0,
    item_serial  BIGINT       NOT NULL DEFAULT 0,
    item_expiry  INTEGER      NOT NULL DEFAULT 0,
    coins        INTEGER      NOT NULL DEFAULT 0,
    b_status     SMALLINT     NOT NULL DEFAULT 0,   -- 0=unread, 1=read
    b_deleted    SMALLINT     NOT NULL DEFAULT 0,   -- 0=active, 1=deleted
    item_taken   SMALLINT     NOT NULL DEFAULT 0,   -- 0=not taken, 1=taken
    send_date    INTEGER      NOT NULL DEFAULT 0,   -- yy*10000+mm*100+dd format
    days_remaining SMALLINT   NOT NULL DEFAULT 30,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_letter_recipient ON letter (recipient_name, b_deleted);
CREATE INDEX IF NOT EXISTS idx_letter_sender ON letter (sender_name);
