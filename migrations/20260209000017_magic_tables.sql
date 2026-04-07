-- Magic system tables
-- Source: MSSQL reference DB (10 tables, 8,106 total rows)
-- C++ structs: _MAGIC_TABLE, _MAGIC_TYPE1 through _MAGIC_TYPE9 (shared/database/structs.h)
-- C++ loaders: MagicTableSet.h, MagicType1Set.h .. MagicType9Set.h

--------------------------------------------------------------------------------
-- MAGIC (base magic/skill table) — 3,880 rows in MSSQL
-- C++ struct: _MAGIC_TABLE
-- Lookup key: MagicNum (iNum in C++)
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic (
    magic_num       INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32), MSSQL: MagicNum (int)
    en_name         VARCHAR(200),                      -- MSSQL: EnName (char 200) — not loaded by C++
    kr_name         VARCHAR(200),                      -- C++: krname (string), MSSQL: KrName (char 200)
    description     VARCHAR(200),                      -- MSSQL: Description (char 200) — not loaded by C++
    t_1             INTEGER,                           -- C++: t_1 (int32)
    before_action   INTEGER,                           -- C++: nBeforeAction (uint32)
    target_action   SMALLINT,                          -- C++: bTargetAction (uint8), MSSQL: tinyint
    self_effect     SMALLINT,                          -- C++: bSelfEffect (uint8), MSSQL: smallint
    flying_effect   SMALLINT,                          -- C++: bFlyingEffect (uint16), MSSQL: smallint
    target_effect   SMALLINT,                          -- C++: iTargetEffect (uint16), MSSQL: smallint
    moral           SMALLINT,                          -- C++: bMoral (uint8), MSSQL: tinyint
    skill_level     SMALLINT,                          -- C++: sSkillLevel (uint16)
    skill           SMALLINT,                          -- C++: sSkill (uint16)
    msp             SMALLINT,                          -- C++: sMsp (uint16)
    hp              SMALLINT,                          -- C++: sHP (uint16)
    s_sp            SMALLINT,                          -- C++: sSp (uint16)
    item_group      SMALLINT,                          -- C++: bItemGroup (uint8), MSSQL: tinyint
    use_item        INTEGER,                           -- C++: iUseItem (uint32)
    cast_time       SMALLINT,                          -- C++: bCastTime (uint8), MSSQL: smallint
    recast_time     SMALLINT,                          -- C++: sReCastTime (uint16)
    success_rate    SMALLINT,                          -- C++: bSuccessRate (uint8), MSSQL: smallint
    type1           SMALLINT,                          -- C++: bType[0] (uint8), MSSQL: tinyint
    type2           SMALLINT,                          -- C++: bType[1] (uint8), MSSQL: tinyint
    "range"         SMALLINT,                          -- C++: sRange (uint16), quoted: reserved word
    etc             SMALLINT,                          -- C++: sEtc (uint16)
    use_standing    SMALLINT,                          -- C++: sUseStanding (uint8), MSSQL: smallint
    skill_check     SMALLINT,                          -- C++: sSkillCheck (uint8), MSSQL: smallint
    icelightrate    SMALLINT                           -- C++: icelightrate (uint16)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE1 (melee attack skills) — 525 rows in MSSQL
-- C++ struct: _MAGIC_TYPE1
-- Lookup key: iNum
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type1 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    hit_type            INTEGER,                           -- C++: bHitType (uint8), MSSQL: Type (int)
    hit_rate            INTEGER,                           -- C++: sHitRate (uint16), MSSQL: HitRate (int)
    hit                 INTEGER,                           -- C++: sHit (uint16), MSSQL: Hit (int)
    add_damage          INTEGER,                           -- C++: sAddDamage (uint16), MSSQL: AddDamage (int)
    combo_type          INTEGER,                           -- C++: bComboType (uint8), MSSQL: ComboType (int)
    combo_count         INTEGER,                           -- C++: bComboCount (uint8), MSSQL: ComboCount (int)
    combo_damage        INTEGER,                           -- C++: sComboDamage (uint16), MSSQL: ComboDamage (int)
    "range"             INTEGER,                           -- C++: sRange (uint16), MSSQL: Range (int)
    delay               INTEGER,                           -- C++: bDelay (uint8), MSSQL: Delay (int)
    add_dmg_perc_to_user INTEGER,                          -- C++: iADPtoUser (uint16), MSSQL: AddDmgPercToUser (int)
    add_dmg_perc_to_npc  INTEGER                           -- C++: iADPtoNPC (uint16), MSSQL: AddDmgPercToNpc (int)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE2 (ranged/archery attack skills) — 91 rows in MSSQL
-- C++ struct: _MAGIC_TYPE2
-- Lookup key: iNum
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type2 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    hit_type            INTEGER,                           -- C++: bHitType (uint8), MSSQL: HitType (int)
    hit_rate            INTEGER,                           -- C++: sHitRate (uint16), MSSQL: HitRate (int)
    add_damage          INTEGER,                           -- C++: sAddDamage (uint16), MSSQL: AddDamage (int)
    add_range           INTEGER,                           -- C++: sAddRange (uint16), MSSQL: AddRange (int)
    need_arrow          INTEGER,                           -- C++: bNeedArrow (uint8), MSSQL: NeedArrow (int)
    add_dmg_perc_to_user SMALLINT,                         -- C++: iADPtoUser (uint16), MSSQL: AddDmgPercToUser (smallint)
    add_dmg_perc_to_npc  SMALLINT                          -- C++: iADPtoNPC (uint16), MSSQL: AddDmgPercToNpc (smallint)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE3 (damage-over-time / direct magic damage) — 1,152 rows in MSSQL
-- C++ struct: _MAGIC_TYPE3
-- Lookup key: iNum
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type3 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    direct_type         INTEGER,                           -- C++: bDirectType (uint8), MSSQL: DirectType (int)
    first_damage        INTEGER,                           -- C++: sFirstDamage (int16), MSSQL: FirstDamage (int)
    time_damage         INTEGER,                           -- C++: sTimeDamage (int16), MSSQL: TimeDamage (int)
    duration            INTEGER,                           -- C++: bDuration (uint8), MSSQL: Duration (int)
    attribute           INTEGER,                           -- C++: bAttribute (uint8), MSSQL: Attribute (int)
    radius              INTEGER,                           -- C++: bRadius (uint8), MSSQL: Radius (int)
    add_dmg_perc_to_user SMALLINT,                         -- C++: iADPtoUser (uint16), MSSQL: AddDmgPercToUser (smallint)
    add_dmg_perc_to_npc  SMALLINT                          -- C++: iADPtoNPC (uint16), MSSQL: AddDmgPercToNpc (smallint)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE4 (buff/debuff skills) — 1,917 rows in MSSQL
-- C++ struct: _MAGIC_TYPE4
-- Lookup key: iNum
-- Largest type table; contains stat modifiers for buffs and debuffs
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type4 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    buff_type           INTEGER,                           -- C++: bBuffType (uint8), MSSQL: BuffType (int)
    radius              INTEGER,                           -- C++: bRadius (uint8), MSSQL: Radius (int)
    duration            INTEGER,                           -- C++: sDuration (uint16), MSSQL: Duration (int)
    attack_speed        INTEGER,                           -- C++: bAttackSpeed (uint8), MSSQL: AttackSpeed (int)
    speed               INTEGER,                           -- C++: bSpeed (uint8), MSSQL: Speed (int)
    ac                  INTEGER,                           -- C++: sAC (int16), MSSQL: AC (int)
    ac_pct              INTEGER,                           -- C++: sACPct (uint16), MSSQL: ACPct (int)
    attack              INTEGER,                           -- C++: bAttack (uint8), MSSQL: Attack (int)
    magic_attack        INTEGER,                           -- C++: bMagicAttack (uint8), MSSQL: MagicAttack (int)
    max_hp              INTEGER,                           -- C++: sMaxHP (uint16), MSSQL: MaxHP (int)
    max_hp_pct          INTEGER,                           -- C++: sMaxHPPct (uint16), MSSQL: MaxHPPct (int)
    max_mp              INTEGER,                           -- C++: sMaxMP (uint16), MSSQL: MaxMP (int)
    max_mp_pct          INTEGER,                           -- C++: sMaxMPPct (uint16), MSSQL: MaxMPPct (int)
    str                 INTEGER,                           -- C++: bStr (int8), MSSQL: Str (int)
    sta                 INTEGER,                           -- C++: bSta (int8), MSSQL: Sta (int)
    dex                 INTEGER,                           -- C++: bDex (int8), MSSQL: Dex (int)
    intel               INTEGER,                           -- C++: bIntel (int8), MSSQL: Intel (int)
    cha                 INTEGER,                           -- C++: bCha (int8), MSSQL: Cha (int)
    fire_r              INTEGER,                           -- C++: bFireR (uint8), MSSQL: FireR (int)
    cold_r              INTEGER,                           -- C++: bColdR (uint8), MSSQL: ColdR (int)
    lightning_r         INTEGER,                           -- C++: bLightningR (uint8), MSSQL: LightningR (int)
    magic_r             INTEGER,                           -- C++: bMagicR (uint8), MSSQL: MagicR (int)
    disease_r           INTEGER,                           -- C++: bDiseaseR (uint8), MSSQL: DiseaseR (int)
    poison_r            INTEGER,                           -- C++: bPoisonR (uint8), MSSQL: PoisonR (int)
    exp_pct             INTEGER,                           -- C++: sExpPct (uint16), MSSQL: ExpPct (int)
    special_amount      INTEGER,                           -- C++: sSpecialAmount (uint16), MSSQL: SpecialAmount (int)
    hit_rate            INTEGER,                           -- C++: bHitRate (uint8), MSSQL: HitRate (int)
    avoid_rate          INTEGER                            -- C++: sAvoidRate (uint16), MSSQL: AvoidRate (int)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE5 (resurrection/recovery skills) — 59 rows in MSSQL
-- C++ struct: _MAGIC_TYPE5
-- Lookup key: iNum
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type5 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    "type"              INTEGER,                           -- C++: bType (uint8), MSSQL: Type (int), quoted: reserved word
    exp_recover         INTEGER,                           -- C++: bExpRecover (uint8), MSSQL: ExpRecover (int)
    need_stone          INTEGER                            -- C++: sNeedStone (uint16), MSSQL: NeedStone (int)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE6 (transformation skills) — 251 rows in MSSQL
-- C++ struct: _MAGIC_TYPE6
-- Lookup key: iNum
-- Includes transform stats (siege weapons, monsters, NPCs, etc.)
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type6 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    "name"              TEXT,                              -- MSSQL: Name (varchar MAX) — not loaded by C++
    description         TEXT,                              -- MSSQL: Description (varchar MAX) — not loaded by C++
    "size"              INTEGER     NOT NULL DEFAULT 0,    -- C++: sSize (uint16), MSSQL: Size (int)
    transform_id        INTEGER     NOT NULL DEFAULT 0,    -- C++: sTransformID (uint16), MSSQL: TransformID (int)
    duration            INTEGER     NOT NULL DEFAULT 0,    -- C++: sDuration (uint16), MSSQL: Duration (int)
    max_hp              INTEGER     NOT NULL DEFAULT 0,    -- C++: sMaxHp (uint16), MSSQL: MaxHp (int)
    max_mp              INTEGER     NOT NULL DEFAULT 0,    -- C++: sMaxMp (uint16), MSSQL: MaxMp (int)
    speed               INTEGER     NOT NULL DEFAULT 0,    -- C++: bSpeed (uint8), MSSQL: Speed (int)
    attack_speed        INTEGER     NOT NULL DEFAULT 0,    -- C++: sAttackSpeed (uint16), MSSQL: AttackSpeed (int)
    total_hit           INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalHit (uint16), MSSQL: TotalHit (int)
    total_ac            INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalAc (uint16), MSSQL: TotalAc (int)
    total_hit_rate      INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalHitRate (uint16), MSSQL: TotalHitRate (int)
    total_evasion_rate  INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalEvasionRate (uint16), MSSQL: TotalEvasionRate (int)
    total_fire_r        INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalFireR (uint16), MSSQL: TotalFireR (int)
    total_cold_r        INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalColdR (uint16), MSSQL: TotalColdR (int)
    total_lightning_r   INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalLightningR (uint16), MSSQL: TotalLightningR (int)
    total_magic_r       INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalMagicR (uint16), MSSQL: TotalMagicR (int)
    total_disease_r     INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalDiseaseR (uint16), MSSQL: TotalDiseaseR (int)
    total_poison_r      INTEGER     NOT NULL DEFAULT 0,    -- C++: sTotalPoisonR (uint16), MSSQL: TotalPoisonR (int)
    class               INTEGER     NOT NULL DEFAULT 0,    -- C++: sClass (uint16), MSSQL: Class (int)
    user_skill_use      INTEGER     NOT NULL DEFAULT 0,    -- C++: bUserSkillUse (uint8), MSSQL: UserSkillUse (int)
    need_item           INTEGER     NOT NULL DEFAULT 0,    -- C++: bNeedItem (uint8), MSSQL: NeedItem (int)
    skill_success_rate  SMALLINT    NOT NULL DEFAULT 0,    -- C++: bSkillSuccessRate (uint8), MSSQL: SkillSuccessRate (tinyint)
    monster_friendly    INTEGER     NOT NULL DEFAULT 0,    -- C++: bMonsterFriendly (uint8), MSSQL: MonsterFriendly (int)
    nation              INTEGER     NOT NULL DEFAULT 0     -- C++: bNation (uint8), MSSQL: Nation (int)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE7 (debuff/crowd-control skills) — 26 rows in MSSQL
-- C++ struct: _MAGIC_TYPE7
-- Lookup key: nIndex (maps to C++ iNum)
-- Note: MSSQL column names use Hungarian notation (by/sh prefixes)
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type7 (
    n_index             INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32), MSSQL: nIndex (int)
    str_name            VARCHAR(30),                       -- MSSQL: strName (char 30) — not loaded by C++
    str_note            VARCHAR(100),                      -- MSSQL: strNote (char 100) — not loaded by C++
    valid_group         SMALLINT    NOT NULL DEFAULT 0,    -- C++: bValidGroup (uint8), MSSQL: byValidGroup (tinyint)
    nation_change       SMALLINT    NOT NULL DEFAULT 0,    -- C++: bNationChange (uint8), MSSQL: byNatoinChange (tinyint)
    monster_num         SMALLINT    NOT NULL DEFAULT 0,    -- C++: sMonsterNum (uint16), MSSQL: shMonsterNum (smallint)
    target_change       SMALLINT    NOT NULL DEFAULT 0,    -- C++: bTargetChange (uint8), MSSQL: byTargetChange (tinyint)
    state_change        SMALLINT    NOT NULL DEFAULT 0,    -- C++: bStateChange (uint8), MSSQL: byStateChange (tinyint)
    radius              SMALLINT    NOT NULL DEFAULT 0,    -- C++: bRadius (uint8), MSSQL: byRadius (tinyint)
    hit_rate            SMALLINT    NOT NULL DEFAULT 0,    -- C++: sHitRate (uint16), MSSQL: shHitrate (smallint)
    duration            SMALLINT    NOT NULL DEFAULT 0,    -- C++: sDuration (uint16), MSSQL: shDuration (smallint)
    damage              SMALLINT    NOT NULL DEFAULT 0,    -- C++: sDamage (uint16), MSSQL: shDamage (smallint)
    vision              SMALLINT    NOT NULL DEFAULT 0,    -- C++: bVision (uint8), MSSQL: byVisoin (tinyint)
    need_item           INTEGER     NOT NULL DEFAULT 0     -- C++: nNeedItem (uint32), MSSQL: nNeedItem (int)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE8 (warp/teleport/resurrection skills) — 121 rows in MSSQL
-- C++ struct: _MAGIC_TYPE8
-- Lookup key: iNum
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type8 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    "name"              VARCHAR(30),                       -- MSSQL: Name (char 30) — not loaded by C++
    description         VARCHAR(100),                      -- MSSQL: Description (char 100) — not loaded by C++
    target              SMALLINT    NOT NULL DEFAULT 0,    -- C++: bTarget (uint8), MSSQL: Target (tinyint)
    radius              SMALLINT    NOT NULL DEFAULT 0,    -- C++: sRadius (uint16), MSSQL: Radius (smallint)
    warp_type           SMALLINT    NOT NULL DEFAULT 0,    -- C++: bWarpType (uint8), MSSQL: WarpType (tinyint)
    exp_recover         SMALLINT    NOT NULL DEFAULT 0,    -- C++: sExpRecover (uint16), MSSQL: ExpRecover (smallint)
    kick_distance       SMALLINT    NOT NULL DEFAULT 0     -- C++: sKickDistance (uint16), MSSQL: KickDistance (smallint)
);

--------------------------------------------------------------------------------
-- MAGIC_TYPE9 (advanced debuff/crowd-control) — 84 rows in MSSQL
-- C++ struct: _MAGIC_TYPE9
-- Lookup key: iNum
-- Similar to TYPE7 but with uint16 fields for radius/vision/damage
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS magic_type9 (
    i_num               INTEGER     NOT NULL PRIMARY KEY,  -- C++: iNum (uint32)
    "name"              TEXT,                              -- MSSQL: Name (varchar MAX) — not loaded by C++
    description         TEXT,                              -- MSSQL: Description (varchar MAX) — not loaded by C++
    valid_group         SMALLINT,                          -- C++: bValidGroup (uint8), MSSQL: ValidGroup (tinyint)
    nation_change       SMALLINT,                          -- C++: bNationChange (uint8), MSSQL: NationChange (tinyint)
    monster_num         INTEGER,                           -- C++: sMonsterNum (uint16), MSSQL: MonsterNum (int)
    target_change       SMALLINT,                          -- C++: bTargetChange (uint8), MSSQL: TargetChange (tinyint)
    state_change        SMALLINT,                          -- C++: bStateChange (uint8), MSSQL: StateChange (tinyint)
    radius              SMALLINT,                          -- C++: sRadius (uint16), MSSQL: Radius (tinyint)
    hit_rate            SMALLINT,                          -- C++: sHitRate (uint16), MSSQL: Hitrate (tinyint)
    duration            INTEGER,                           -- C++: sDuration (uint16), MSSQL: Duration (int)
    add_damage          SMALLINT,                          -- C++: sDamage (uint16), MSSQL: AddDamage (tinyint)
    vision              SMALLINT,                          -- C++: sVision (uint16), MSSQL: Vision (tinyint)
    need_item           SMALLINT                           -- C++: nNeedItem (uint32), MSSQL: NeedItem (tinyint)
);

-- Indexes for common lookups (type1/type2 in magic table → join to type subtables)
CREATE INDEX IF NOT EXISTS idx_magic_type1 ON magic (type1);
CREATE INDEX IF NOT EXISTS idx_magic_type2 ON magic (type2);
