-- Knights Cape Castellan Bonus table
-- Source: MSSQL KNIGHTS_CAPE_CASTELLAN_BONUS (2 rows)
-- C++ Reference: CCapeCastellanBonusSet in CapeCastellanBonusSet.h

CREATE TABLE IF NOT EXISTS knights_cape_castellan_bonus (
    bonus_type      SMALLINT    NOT NULL PRIMARY KEY,
    type_name       VARCHAR(50) NOT NULL DEFAULT '',
    ac_bonus        SMALLINT    NOT NULL DEFAULT 0,
    hp_bonus        SMALLINT    NOT NULL DEFAULT 0,
    mp_bonus        SMALLINT    NOT NULL DEFAULT 0,
    str_bonus       SMALLINT    NOT NULL DEFAULT 0,
    sta_bonus       SMALLINT    NOT NULL DEFAULT 0,
    dex_bonus       SMALLINT    NOT NULL DEFAULT 0,
    int_bonus       SMALLINT    NOT NULL DEFAULT 0,
    cha_bonus       SMALLINT    NOT NULL DEFAULT 0,
    flame_resist    SMALLINT    NOT NULL DEFAULT 0,
    glacier_resist  SMALLINT    NOT NULL DEFAULT 0,
    lightning_resist SMALLINT   NOT NULL DEFAULT 0,
    magic_resist    SMALLINT    NOT NULL DEFAULT 0,
    disease_resist  SMALLINT    NOT NULL DEFAULT 0,
    poison_resist   SMALLINT    NOT NULL DEFAULT 0,
    xp_bonus_pct    SMALLINT    NOT NULL DEFAULT 0,
    coin_bonus_pct  SMALLINT    NOT NULL DEFAULT 0,
    ap_bonus_pct    SMALLINT    NOT NULL DEFAULT 0,
    ac_bonus_pct    SMALLINT    NOT NULL DEFAULT 0,
    max_weight_bonus SMALLINT   NOT NULL DEFAULT 0,
    np_bonus        SMALLINT    NOT NULL DEFAULT 0
);

INSERT INTO knights_cape_castellan_bonus
    (bonus_type, type_name, ac_bonus, hp_bonus, mp_bonus,
     str_bonus, sta_bonus, dex_bonus, int_bonus, cha_bonus,
     flame_resist, glacier_resist, lightning_resist, magic_resist,
     disease_resist, poison_resist,
     xp_bonus_pct, coin_bonus_pct, ap_bonus_pct, ac_bonus_pct,
     max_weight_bonus, np_bonus)
VALUES
    (1, 'Castellan Cape Bonus Type 1', 0, 300, 0,
     0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0),
    (2, 'Castellan Cape Bonus Type 2', 200, 500, 500,
     5, 5, 5, 5, 0,
     50, 50, 50, 0, 0, 0,
     0, 0, 3, 0, 0, 10)
ON CONFLICT (bonus_type) DO NOTHING;
