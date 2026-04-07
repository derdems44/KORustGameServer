-- ITEM_EXCHANGE_EXP: Item exchange experience rules.
-- MSSQL source: ITEM_EXCHANGE_EXP (204 rows).
-- C++ Reference: CGameServerDlg::m_ItemExchangeExpArray
CREATE TABLE IF NOT EXISTS item_exchange_exp (
    n_index              INTEGER  PRIMARY KEY,
    random_flag          SMALLINT DEFAULT 0,
    exchange_item_num1   INTEGER  DEFAULT 0,
    exchange_item_count1 INTEGER  DEFAULT 0,
    exchange_item_num2   INTEGER  DEFAULT 0,
    exchange_item_count2 INTEGER  DEFAULT 0,
    exchange_item_num3   INTEGER  DEFAULT 0,
    exchange_item_count3 INTEGER  DEFAULT 0,
    exchange_item_num4   INTEGER  DEFAULT 0,
    exchange_item_count4 INTEGER  DEFAULT 0,
    exchange_item_num5   INTEGER  DEFAULT 0,
    exchange_item_count5 INTEGER  DEFAULT 0,
    exchange_item_time1  INTEGER  DEFAULT 0,
    exchange_item_time2  INTEGER  DEFAULT 0,
    exchange_item_time3  INTEGER  DEFAULT 0,
    exchange_item_time4  INTEGER  DEFAULT 0,
    exchange_item_time5  INTEGER  DEFAULT 0
);

-- ITEM_GIVE_EXCHANGE: Exchange giving rules (rob items -> give items).
-- MSSQL source: ITEM_GIVE_EXCHANGE (661 rows, 126 columns).
-- Normalized: rob/give items stored as arrays instead of 25 individual columns each.
-- C++ Reference: CGameServerDlg::m_ItemGiveExchangeArray
CREATE TABLE IF NOT EXISTS item_give_exchange (
    exchange_index   INTEGER  NOT NULL PRIMARY KEY,
    rob_item_ids     INTEGER[] NOT NULL DEFAULT '{}',
    rob_item_counts  INTEGER[] NOT NULL DEFAULT '{}',
    give_item_ids    INTEGER[] NOT NULL DEFAULT '{}',
    give_item_counts INTEGER[] NOT NULL DEFAULT '{}',
    give_item_times  INTEGER[] NOT NULL DEFAULT '{}'
);

-- ITEM_RIGHT_CLICK_EXCHANGE: Maps item IDs to right-click exchange opcodes.
-- MSSQL source: ITEM_RIGHT_CLICK_EXCHANGE (96 rows).
-- C++ Reference: CGameServerDlg::m_ItemRightClickExchangeArray
CREATE TABLE IF NOT EXISTS item_right_click_exchange (
    item_id  INTEGER  NOT NULL PRIMARY KEY,
    opcode   SMALLINT NOT NULL DEFAULT 0
);

-- ITEM_RIGHT_EXCHANGE: Right-click exchange definitions (item -> rewards).
-- MSSQL source: ITEM_RIGHT_EXCHANGE (66 rows, 80 columns).
-- Normalized: exchange items stored as arrays.
-- C++ Reference: CGameServerDlg::m_ItemRightExchangeArray
CREATE TABLE IF NOT EXISTS item_right_exchange (
    item_id          INTEGER     PRIMARY KEY,
    str_name         VARCHAR(50) DEFAULT '',
    exchange_type    SMALLINT    DEFAULT 0,
    description      VARCHAR(20) DEFAULT '',
    exchange_count   INTEGER     DEFAULT 0,
    exchange_items   INTEGER[]   NOT NULL DEFAULT '{}',
    exchange_counts  INTEGER[]   NOT NULL DEFAULT '{}',
    expiration_times INTEGER[]   NOT NULL DEFAULT '{}'
);
