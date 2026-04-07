-- Server-wide configuration settings (single-row).
-- C++ Reference: _SERVER_SETTING struct in GameDefine.h:3859
-- Source: MSSQL SERVER_SETTINGS table (1 row)

CREATE TABLE IF NOT EXISTS server_settings (
    server_no           SMALLINT    NOT NULL DEFAULT 1 PRIMARY KEY,
    maximum_level       SMALLINT    NOT NULL DEFAULT 83,
    drop_notice         SMALLINT    NOT NULL DEFAULT 1,
    upgrade_notice      SMALLINT    NOT NULL DEFAULT 1,
    user_max_upgrade    SMALLINT    NOT NULL DEFAULT 32,
    merchant_view       SMALLINT    NOT NULL DEFAULT 0,
    clan_bank_premium   SMALLINT    NOT NULL DEFAULT 0,
    auto_royal_g1       SMALLINT    NOT NULL DEFAULT 1,
    auto_basic_skill    SMALLINT    NOT NULL DEFAULT 0,
    auto_master_skill   SMALLINT    NOT NULL DEFAULT 1,
    auto_quest_skill    SMALLINT    NOT NULL DEFAULT 1,
    auto_wanted         SMALLINT    NOT NULL DEFAULT 0,
    loot_genie_premium  SMALLINT    NOT NULL DEFAULT 0,
    merchant_min_cash   SMALLINT    NOT NULL DEFAULT 1000,
    trash_item          BOOLEAN     NOT NULL DEFAULT FALSE,
    online_give_cash    BOOLEAN     NOT NULL DEFAULT FALSE,
    online_cash_time    INTEGER     NOT NULL DEFAULT 0,
    flash_time          INTEGER     NOT NULL DEFAULT 180,
    free_skill_stat     SMALLINT    NOT NULL DEFAULT 1,
    merchant_level      SMALLINT    NOT NULL DEFAULT 1,
    trade_level         SMALLINT    NOT NULL DEFAULT 1,
    chaotic_coins       INTEGER     NOT NULL DEFAULT 0,
    mute_level          SMALLINT    NOT NULL DEFAULT 0,
    monsterstone_status BOOLEAN     NOT NULL DEFAULT TRUE,
    new_monsterstone    SMALLINT    NOT NULL DEFAULT 0,
    etrafa_item1        INTEGER     NOT NULL DEFAULT 0,
    etrafa_count1       INTEGER     NOT NULL DEFAULT 0,
    etrafa_item2        INTEGER     NOT NULL DEFAULT 0,
    etrafa_count2       INTEGER     NOT NULL DEFAULT 0,
    etrafa_item3        INTEGER     NOT NULL DEFAULT 0,
    etrafa_count3       INTEGER     NOT NULL DEFAULT 0,
    max_player_hp       SMALLINT    NOT NULL DEFAULT 14000,
    welcome_msg         TEXT        NOT NULL DEFAULT '',
    perk_coins          INTEGER     NOT NULL DEFAULT 0,
    premium_id          SMALLINT    NOT NULL DEFAULT 0,
    premium_time        SMALLINT    NOT NULL DEFAULT 0,
    max_blessing_up     SMALLINT    NOT NULL DEFAULT 10,
    max_blessing_up_reb SMALLINT    NOT NULL DEFAULT 30,
    give_genie_hour     SMALLINT    NOT NULL DEFAULT 0
);

-- Seed with actual MSSQL data
INSERT INTO server_settings (
    server_no, maximum_level, drop_notice, upgrade_notice, user_max_upgrade,
    merchant_view, clan_bank_premium, auto_royal_g1, auto_basic_skill,
    auto_master_skill, auto_quest_skill, auto_wanted, loot_genie_premium,
    merchant_min_cash, trash_item, online_give_cash, online_cash_time,
    flash_time, free_skill_stat, merchant_level, trade_level, chaotic_coins,
    mute_level, monsterstone_status, new_monsterstone,
    etrafa_item1, etrafa_count1, etrafa_item2, etrafa_count2,
    etrafa_item3, etrafa_count3, max_player_hp, welcome_msg,
    perk_coins, premium_id, premium_time, max_blessing_up,
    max_blessing_up_reb, give_genie_hour
) VALUES (
    1, 83, 1, 1, 32,
    0, 0, 1, 0,
    1, 1, 0, 0,
    1000, FALSE, FALSE, 0,
    180, 1, 1, 1, 0,
    0, TRUE, 0,
    0, 0, 0, 0,
    0, 0, 14000, '',
    0, 0, 0, 10,
    30, 0
) ON CONFLICT (server_no) DO NOTHING;
