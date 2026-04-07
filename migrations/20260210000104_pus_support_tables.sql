-- PUS support tables: prepaid cards, promotion codes, premium gift items
-- Source: MSSQL PPCARD_LIST (2 rows), PPCARD_PROMOTION_CODE (1 row), PREMIUM_GIFT_ITEM (1 row)

-- Prepaid card definitions (rarely used, schema-only with minimal seed)
CREATE TABLE IF NOT EXISTS ppcard_list (
    pp_key_code     VARCHAR(20) NOT NULL PRIMARY KEY,
    knight_cash     INTEGER     NOT NULL DEFAULT 0,
    tl_balance      INTEGER     NOT NULL DEFAULT 0,
    status          INTEGER     NOT NULL DEFAULT 0,
    account_id      VARCHAR(21),
    user_id         VARCHAR(21),
    update_date     VARCHAR(200),
    cash_type       SMALLINT    NOT NULL DEFAULT 1,
    description     VARCHAR(500),
    all_user        BOOLEAN     NOT NULL DEFAULT FALSE
);

-- Promotion code system
CREATE TABLE IF NOT EXISTS ppcard_promotion_code (
    n_index             INTEGER     NOT NULL PRIMARY KEY,
    promotion_code      VARCHAR(21) NOT NULL,
    note                VARCHAR(50) NOT NULL DEFAULT '',
    item_id             INTEGER     NOT NULL DEFAULT 0,
    item_count          INTEGER     NOT NULL DEFAULT 0,
    item_remaining_time INTEGER     NOT NULL DEFAULT 0,
    duration            INTEGER     NOT NULL DEFAULT 0,
    item_flag           SMALLINT    NOT NULL DEFAULT 0
);

-- Premium gift items (bonus items given with premium purchases)
CREATE TABLE IF NOT EXISTS premium_gift_item (
    id              INTEGER     NOT NULL PRIMARY KEY,
    premium_type    SMALLINT,
    bonus_item_num  INTEGER,
    count           SMALLINT,
    sender          VARCHAR(21),
    subject         VARCHAR(50),
    message         VARCHAR(128),
    item_name       VARCHAR(128)
);
