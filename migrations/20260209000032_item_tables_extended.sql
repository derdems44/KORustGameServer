-- Extended item-related reference tables
-- Tables: npc_item, item_upgrade, make_weapon, make_defensive,
--         make_item_gradecode, make_item_larecode, make_item_group,
--         make_item_group_random, rental_item
-- C++ Reference: shared/database/ headers for each table

-----------------------------------------------
-- npc_item: NPC drop tables (12 item slots per NPC)
-- MSSQL source: K_NPC_ITEM
-- C++ Reference: NpcItemSet.h (_K_NPC_ITEM)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS npc_item (
    s_index     SMALLINT  NOT NULL PRIMARY KEY,
    item01      INTEGER   DEFAULT 0,
    percent01   SMALLINT  DEFAULT 0,
    item02      INTEGER   DEFAULT 0,
    percent02   SMALLINT  DEFAULT 0,
    item03      INTEGER   DEFAULT 0,
    percent03   SMALLINT  DEFAULT 0,
    item04      INTEGER   DEFAULT 0,
    percent04   SMALLINT  DEFAULT 0,
    item05      INTEGER   DEFAULT 0,
    percent05   SMALLINT  DEFAULT 0,
    item06      INTEGER   DEFAULT 0,
    percent06   SMALLINT  DEFAULT 0,
    item07      INTEGER   DEFAULT 0,
    percent07   SMALLINT  DEFAULT 0,
    item08      INTEGER   DEFAULT 0,
    percent08   SMALLINT  DEFAULT 0,
    item09      INTEGER   DEFAULT 0,
    percent09   SMALLINT  DEFAULT 0,
    item10      INTEGER   DEFAULT 0,
    percent10   SMALLINT  DEFAULT 0,
    item11      INTEGER   DEFAULT 0,
    percent11   SMALLINT  DEFAULT 0,
    item12      INTEGER   DEFAULT 0,
    percent12   SMALLINT  DEFAULT 0
);

-----------------------------------------------
-- item_upgrade: Item upgrade recipes with NPC, materials, and rates
-- MSSQL source: ITEM_UPGRADE
-- C++ Reference: ItemUpgradeSet.h (_ITEM_UPGRADE)
-- Constants: MAX_ITEMS_REQ_FOR_UPGRADE = 8
-----------------------------------------------
CREATE TABLE IF NOT EXISTS item_upgrade (
    n_index       INTEGER   NOT NULL PRIMARY KEY,
    npc_num       SMALLINT  DEFAULT 0,
    origin_type   SMALLINT  DEFAULT 0,
    origin_item   INTEGER   DEFAULT 0,
    req_item1     INTEGER   DEFAULT 0,
    req_item2     INTEGER   DEFAULT 0,
    req_item3     INTEGER   DEFAULT 0,
    req_item4     INTEGER   DEFAULT 0,
    req_item5     INTEGER   DEFAULT 0,
    req_item6     INTEGER   DEFAULT 0,
    req_item7     INTEGER   DEFAULT 0,
    req_item8     INTEGER   DEFAULT 0,
    req_noah      INTEGER   DEFAULT 0,
    rate_type     SMALLINT  DEFAULT 0,
    gen_rate      SMALLINT  DEFAULT 0,
    trina_rate    SMALLINT  DEFAULT 0,
    karivdis_rate SMALLINT  DEFAULT 0,
    give_item     INTEGER   DEFAULT 0
);

-----------------------------------------------
-- make_weapon: Weapon crafting templates (12 class columns)
-- MSSQL source: MAKE_WEAPON
-- C++ Reference: MakeWeaponTableSet.h (_MAKE_WEAPON)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_weapon (
    by_level   SMALLINT  NOT NULL PRIMARY KEY,
    class_1    SMALLINT  DEFAULT 0,
    class_2    SMALLINT  DEFAULT 0,
    class_3    SMALLINT  DEFAULT 0,
    class_4    SMALLINT  DEFAULT 0,
    class_5    SMALLINT  DEFAULT 0,
    class_6    SMALLINT  DEFAULT 0,
    class_7    SMALLINT  DEFAULT 0,
    class_8    SMALLINT  DEFAULT 0,
    class_9    SMALLINT  DEFAULT 0,
    class_10   SMALLINT  DEFAULT 0,
    class_11   SMALLINT  DEFAULT 0,
    class_12   SMALLINT  DEFAULT 0
);

-----------------------------------------------
-- make_defensive: Defensive crafting templates (7 class columns)
-- MSSQL source: MAKE_DEFENSIVE
-- C++ Reference: MakeDefensiveTableSet.h (_MAKE_WEAPON reused)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_defensive (
    by_level   SMALLINT  NOT NULL PRIMARY KEY,
    class_1    SMALLINT  DEFAULT 0,
    class_2    SMALLINT  DEFAULT 0,
    class_3    SMALLINT  DEFAULT 0,
    class_4    SMALLINT  DEFAULT 0,
    class_5    SMALLINT  DEFAULT 0,
    class_6    SMALLINT  DEFAULT 0,
    class_7    SMALLINT  DEFAULT 0
);

-----------------------------------------------
-- make_item_gradecode: Crafting grade code table
-- MSSQL source: MAKE_ITEM_GRADECODE
-- C++ Reference: MakeGradeItemTableSet.h (_MAKE_ITEM_GRADE_CODE)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_item_gradecode (
    item_index  SMALLINT  NOT NULL PRIMARY KEY,
    grade_1     SMALLINT  DEFAULT 0,
    grade_2     SMALLINT  DEFAULT 0,
    grade_3     SMALLINT  DEFAULT 0,
    grade_4     SMALLINT  DEFAULT 0,
    grade_5     SMALLINT  DEFAULT 0,
    grade_6     SMALLINT  DEFAULT 0,
    grade_7     SMALLINT  DEFAULT 0,
    grade_8     SMALLINT  DEFAULT 0,
    grade_9     SMALLINT  DEFAULT 0
);

-----------------------------------------------
-- make_item_larecode: Crafting rarity code table
-- MSSQL source: MAKE_ITEM_LARECODE
-- C++ Reference: MakeLareItemTableSet.h (_MAKE_ITEM_LARE_CODE)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_item_larecode (
    level_grade   SMALLINT  NOT NULL PRIMARY KEY,
    lare_item     SMALLINT  DEFAULT 0,
    magic_item    SMALLINT  DEFAULT 0,
    general_item  SMALLINT  DEFAULT 0
);

-----------------------------------------------
-- make_item_group: Crafting item groups (up to 200 items)
-- MSSQL source: MAKE_ITEM_GROUP
-- C++ Reference: MakeItemGroupSet.h (_MAKE_ITEM_GROUP)
-- Note: Normalized to avoid 200-column table. Uses JSONB array.
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_item_group (
    group_num   INTEGER   NOT NULL PRIMARY KEY,
    items       INTEGER[] NOT NULL DEFAULT '{}'
);

-----------------------------------------------
-- make_item_group_random: Random crafting item group mapping
-- MSSQL source: MAKE_ITEM_GROUP_RANDOM
-- C++ Reference: MakeItemGroupSet.h (_MAKE_ITEM_GROUP_RANDOM)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS make_item_group_random (
    n_index     INTEGER   NOT NULL PRIMARY KEY,
    item_id     INTEGER   DEFAULT 0,
    group_no    INTEGER   DEFAULT 0
);
CREATE INDEX idx_make_item_group_random_group ON make_item_group_random (group_no);

-----------------------------------------------
-- rental_item: Rental system items
-- MSSQL source: RENTAL_ITEM
-- C++ Reference: RentalItemSet.h (_RENTAL_ITEM)
-----------------------------------------------
CREATE TABLE IF NOT EXISTS rental_item (
    rental_index       INTEGER      NOT NULL PRIMARY KEY,
    item_index         INTEGER      DEFAULT 0,
    durability         SMALLINT     DEFAULT 0,
    serial_number      BIGINT       DEFAULT 0,
    reg_type           SMALLINT     DEFAULT 0,
    item_type          SMALLINT     DEFAULT 0,
    item_class         SMALLINT     DEFAULT 0,
    rental_time        SMALLINT     DEFAULT 0,
    rental_money       INTEGER      DEFAULT 0,
    lender_char_id     VARCHAR(21)  DEFAULT '',
    borrower_char_id   VARCHAR(21)  DEFAULT ''
);
CREATE INDEX idx_rental_item_lender ON rental_item (lender_char_id);
CREATE INDEX idx_rental_item_borrower ON rental_item (borrower_char_id);
