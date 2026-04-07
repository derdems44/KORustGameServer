-- Bot System Tables
-- C++ Reference: CDBAgent::LoadBotTable(), CDBAgent::LoadBotHandlerMerchantTable()
-- MSSQL Source: BOT_HANDLER_FARM (2269 rows), BOT_HANDLER_MERCHANT (82 rows),
--              BOT_MERCHANT_DATA (60 rows), USER_BOTS (207 rows),
--              BOT_KNIGHTS_RANK (200 rows), BOT_PERSONAL_RANK (200 rows)

-- ═══════════════════════════════════════════════════════════════════
-- BOT_HANDLER_FARM: Farm bot population data (character-like records)
-- C++ struct: _BOT_DATA loaded into m_ArtificialIntelligenceArray
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS bot_handler_farm (
    id              INTEGER NOT NULL,
    str_user_id     VARCHAR(21) NOT NULL,
    nation          SMALLINT NOT NULL DEFAULT 0,
    race            SMALLINT NOT NULL DEFAULT 0,
    class           SMALLINT NOT NULL DEFAULT 0,
    hair_rgb        INTEGER NOT NULL DEFAULT 0,
    level           SMALLINT NOT NULL DEFAULT 1,
    face            SMALLINT NOT NULL DEFAULT 0,
    knights         SMALLINT NOT NULL DEFAULT 0,
    fame            SMALLINT NOT NULL DEFAULT 0,
    zone            SMALLINT NOT NULL DEFAULT 0,
    px              INTEGER NOT NULL DEFAULT 0,
    pz              INTEGER NOT NULL DEFAULT 0,
    py              INTEGER NOT NULL DEFAULT 0,
    str_item        BYTEA,
    cover_title     INTEGER NOT NULL DEFAULT 0,
    reb_level       SMALLINT NOT NULL DEFAULT 0,
    str_skill       BYTEA,
    gold            INTEGER NOT NULL DEFAULT 0,
    points          SMALLINT NOT NULL DEFAULT 0,
    strong          SMALLINT NOT NULL DEFAULT 0,
    sta             SMALLINT NOT NULL DEFAULT 0,
    dex             SMALLINT NOT NULL DEFAULT 0,
    intel           SMALLINT NOT NULL DEFAULT 0,
    cha             SMALLINT NOT NULL DEFAULT 0,
    loyalty         INTEGER NOT NULL DEFAULT 0,
    loyalty_monthly INTEGER NOT NULL DEFAULT 0,
    donated_np      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_bot_handler_farm_zone ON bot_handler_farm(zone);
CREATE INDEX IF NOT EXISTS idx_bot_handler_farm_nation ON bot_handler_farm(nation);

-- ═══════════════════════════════════════════════════════════════════
-- BOT_HANDLER_MERCHANT: Merchant bot item templates
-- C++ struct: _BOT_MERCHANT_ITEM loaded into m_ArtificialMerchantArray
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS bot_handler_merchant (
    s_index             SMALLINT NOT NULL,
    bot_merchant_type   SMALLINT NOT NULL DEFAULT 0,
    bot_item_num        VARCHAR(240) NOT NULL DEFAULT '',
    bot_item_count      VARCHAR(240) NOT NULL DEFAULT '',
    bot_item_price      VARCHAR(240) NOT NULL DEFAULT '',
    bot_merchant_message VARCHAR(250),
    PRIMARY KEY (s_index)
);

-- ═══════════════════════════════════════════════════════════════════
-- BOT_MERCHANT_DATA: Pre-configured merchant bot stall data (12 item slots)
-- C++ struct: _BOT_SAVE_DATA
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS bot_merchant_data (
    n_index         INTEGER NOT NULL,
    advert_message  VARCHAR(40),
    n_num1          INTEGER NOT NULL DEFAULT 0,
    n_price1        INTEGER NOT NULL DEFAULT 0,
    s_count1        INTEGER NOT NULL DEFAULT 0,
    s_duration1     INTEGER NOT NULL DEFAULT 0,
    is_kc1          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num2          INTEGER NOT NULL DEFAULT 0,
    n_price2        INTEGER NOT NULL DEFAULT 0,
    s_count2        INTEGER NOT NULL DEFAULT 0,
    s_duration2     INTEGER NOT NULL DEFAULT 0,
    is_kc2          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num3          INTEGER NOT NULL DEFAULT 0,
    n_price3        INTEGER NOT NULL DEFAULT 0,
    s_count3        INTEGER NOT NULL DEFAULT 0,
    s_duration3     INTEGER NOT NULL DEFAULT 0,
    is_kc3          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num4          INTEGER NOT NULL DEFAULT 0,
    n_price4        INTEGER NOT NULL DEFAULT 0,
    s_count4        INTEGER NOT NULL DEFAULT 0,
    s_duration4     INTEGER NOT NULL DEFAULT 0,
    is_kc4          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num5          INTEGER NOT NULL DEFAULT 0,
    n_price5        INTEGER NOT NULL DEFAULT 0,
    s_count5        INTEGER NOT NULL DEFAULT 0,
    s_duration5     INTEGER NOT NULL DEFAULT 0,
    is_kc5          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num6          INTEGER NOT NULL DEFAULT 0,
    n_price6        INTEGER NOT NULL DEFAULT 0,
    s_count6        INTEGER NOT NULL DEFAULT 0,
    s_duration6     INTEGER NOT NULL DEFAULT 0,
    is_kc6          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num7          INTEGER NOT NULL DEFAULT 0,
    n_price7        INTEGER NOT NULL DEFAULT 0,
    s_count7        INTEGER NOT NULL DEFAULT 0,
    s_duration7     INTEGER NOT NULL DEFAULT 0,
    is_kc7          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num8          INTEGER NOT NULL DEFAULT 0,
    n_price8        INTEGER NOT NULL DEFAULT 0,
    s_count8        INTEGER NOT NULL DEFAULT 0,
    s_duration8     INTEGER NOT NULL DEFAULT 0,
    is_kc8          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num9          INTEGER NOT NULL DEFAULT 0,
    n_price9        INTEGER NOT NULL DEFAULT 0,
    s_count9        INTEGER NOT NULL DEFAULT 0,
    s_duration9     INTEGER NOT NULL DEFAULT 0,
    is_kc9          BOOLEAN NOT NULL DEFAULT FALSE,
    n_num10         INTEGER NOT NULL DEFAULT 0,
    n_price10       INTEGER NOT NULL DEFAULT 0,
    s_count10       INTEGER NOT NULL DEFAULT 0,
    s_duration10    INTEGER NOT NULL DEFAULT 0,
    is_kc10         BOOLEAN NOT NULL DEFAULT FALSE,
    n_num11         INTEGER NOT NULL DEFAULT 0,
    n_price11       INTEGER NOT NULL DEFAULT 0,
    s_count11       INTEGER NOT NULL DEFAULT 0,
    s_duration11    INTEGER NOT NULL DEFAULT 0,
    is_kc11         BOOLEAN NOT NULL DEFAULT FALSE,
    n_num12         INTEGER NOT NULL DEFAULT 0,
    n_price12       INTEGER NOT NULL DEFAULT 0,
    s_count12       INTEGER NOT NULL DEFAULT 0,
    s_duration12    INTEGER NOT NULL DEFAULT 0,
    is_kc12         BOOLEAN NOT NULL DEFAULT FALSE,
    px              INTEGER NOT NULL DEFAULT 0,
    pz              INTEGER NOT NULL DEFAULT 0,
    py              INTEGER NOT NULL DEFAULT 0,
    minute          INTEGER NOT NULL DEFAULT 0,
    zone            INTEGER NOT NULL DEFAULT 0,
    s_direction     INTEGER NOT NULL DEFAULT 0,
    merchant_type   SMALLINT NOT NULL DEFAULT 1,
    PRIMARY KEY (n_index)
);

-- ═══════════════════════════════════════════════════════════════════
-- USER_BOTS: Individual user bot definitions (similar to farm bots but no loyalty)
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS user_bots (
    id              INTEGER NOT NULL,
    str_user_id     VARCHAR(21) NOT NULL,
    nation          SMALLINT NOT NULL DEFAULT 0,
    race            SMALLINT NOT NULL DEFAULT 0,
    class           SMALLINT NOT NULL DEFAULT 0,
    hair_rgb        INTEGER NOT NULL DEFAULT 0,
    level           SMALLINT NOT NULL DEFAULT 1,
    face            SMALLINT NOT NULL DEFAULT 0,
    knights         SMALLINT NOT NULL DEFAULT 0,
    fame            SMALLINT NOT NULL DEFAULT 0,
    zone            SMALLINT NOT NULL DEFAULT 0,
    px              INTEGER NOT NULL DEFAULT 0,
    pz              INTEGER NOT NULL DEFAULT 0,
    py              INTEGER NOT NULL DEFAULT 0,
    str_item        BYTEA,
    cover_title     INTEGER NOT NULL DEFAULT 0,
    reb_level       SMALLINT NOT NULL DEFAULT 0,
    str_skill       BYTEA,
    gold            INTEGER NOT NULL DEFAULT 0,
    points          SMALLINT NOT NULL DEFAULT 0,
    strong          SMALLINT NOT NULL DEFAULT 0,
    sta             SMALLINT NOT NULL DEFAULT 0,
    dex             SMALLINT NOT NULL DEFAULT 0,
    intel           SMALLINT NOT NULL DEFAULT 0,
    cha             SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS idx_user_bots_zone ON user_bots(zone);

-- ═══════════════════════════════════════════════════════════════════
-- BOT_KNIGHTS_RANK: Bot knights ranking data (dual nation)
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS bot_knights_rank (
    sh_index            SMALLINT NOT NULL,
    str_name            VARCHAR(21) NOT NULL DEFAULT '',
    str_elmo_user_id    VARCHAR(21),
    str_elmo_knights_name VARCHAR(21),
    s_elmo_knights      SMALLINT DEFAULT 0,
    n_elmo_loyalty      INTEGER,
    str_karus_user_id   VARCHAR(21),
    str_karus_knights_name VARCHAR(21),
    s_karus_knights     SMALLINT DEFAULT 0,
    n_karus_loyalty     INTEGER,
    n_money             INTEGER,
    PRIMARY KEY (sh_index)
);

-- ═══════════════════════════════════════════════════════════════════
-- BOT_PERSONAL_RANK: Bot personal ranking data (dual nation)
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS bot_personal_rank (
    n_rank              SMALLINT NOT NULL,
    str_rank_name       VARCHAR(21) NOT NULL DEFAULT '',
    n_elmo_up           SMALLINT NOT NULL DEFAULT 0,
    str_elmo_user_id    VARCHAR(21),
    str_elmo_clan_name  VARCHAR(21),
    s_elmo_knights      SMALLINT,
    n_elmo_loyalty_monthly INTEGER,
    n_elmo_check        INTEGER NOT NULL DEFAULT 0,
    n_karus_up          SMALLINT NOT NULL DEFAULT 0,
    str_karus_user_id   VARCHAR(21),
    str_karus_clan_name VARCHAR(21),
    s_karus_knights     SMALLINT,
    n_karus_loyalty_monthly INTEGER,
    n_karus_check       INTEGER NOT NULL DEFAULT 0,
    n_salary            INTEGER NOT NULL DEFAULT 0,
    update_date         TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (n_rank)
);

-- ═══════════════════════════════════════════════════════════════════
-- MERCHANT_BOT_INFO: Merchant bot spawn info (empty in MSSQL, schema only)
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS merchant_bot_info (
    n_index     INTEGER NOT NULL,
    type        SMALLINT NOT NULL DEFAULT 0,
    set_x       SMALLINT NOT NULL DEFAULT 0,
    set_y       INTEGER NOT NULL DEFAULT 0,
    set_z       SMALLINT NOT NULL DEFAULT 0,
    b_zone_id   SMALLINT NOT NULL DEFAULT 0,
    direction   INTEGER NOT NULL DEFAULT 0,
    is_buy      BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (n_index)
);

-- ═══════════════════════════════════════════════════════════════════
-- MERCHANT_BOT_ITEM: Merchant bot item pool (empty in MSSQL, schema only)
-- ═══════════════════════════════════════════════════════════════════
CREATE TABLE IF NOT EXISTS merchant_bot_item (
    item_id         INTEGER NOT NULL DEFAULT 0,
    min_item_count  INTEGER NOT NULL DEFAULT 0,
    max_item_count  INTEGER NOT NULL DEFAULT 0,
    min_price       INTEGER NOT NULL DEFAULT 0,
    max_price       INTEGER NOT NULL DEFAULT 0,
    min_kc          INTEGER NOT NULL DEFAULT 0,
    max_kc          INTEGER NOT NULL DEFAULT 0,
    type            SMALLINT NOT NULL DEFAULT 0,
    money_type      SMALLINT NOT NULL DEFAULT 0
);
