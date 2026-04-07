//! Magic system models — maps to the 10 `magic*` PostgreSQL tables.
//! - `shared/database/MagicTableSet.h` — `_MAGIC_TABLE` struct
//! - `shared/database/MagicType1Set.h` through `MagicType9Set.h`
//! The magic system stores skill definitions and type-specific parameters
//! across a master table (`magic`) and 9 type sub-tables (`magic_type1`
//! through `magic_type9`). Each type corresponds to a category of skill
//! behaviour: melee, ranged, DOT, buff/debuff, resurrection, transform,
//! crowd-control, teleport, and advanced CC.

/// Master magic/skill definition row.
/// Keyed by `magic_num`. Contains targeting, cost, cast-time, and type
/// routing fields. The `type1`/`type2` columns determine which sub-table
/// to join for type-specific parameters.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicRow {
    /// Unique magic/skill identifier.
    pub magic_num: i32,
    /// English name (not loaded by C++ runtime).
    pub en_name: Option<String>,
    /// Korean name (C++: krname).
    pub kr_name: Option<String>,
    /// Description text (not loaded by C++ runtime).
    pub description: Option<String>,
    /// Type parameter 1 (C++: t_1).
    pub t_1: Option<i32>,
    /// Pre-cast action ID (C++: nBeforeAction).
    pub before_action: Option<i32>,
    /// Target action type (C++: bTargetAction).
    pub target_action: Option<i16>,
    /// Self visual effect (C++: bSelfEffect).
    pub self_effect: Option<i16>,
    /// Projectile/flying effect (C++: bFlyingEffect).
    pub flying_effect: Option<i16>,
    /// Target visual effect (C++: iTargetEffect).
    pub target_effect: Option<i16>,
    /// Moral alignment requirement (C++: bMoral).
    pub moral: Option<i16>,
    /// Required skill level (C++: sSkillLevel).
    pub skill_level: Option<i16>,
    /// Skill tree identifier (C++: sSkill).
    pub skill: Option<i16>,
    /// MP cost (C++: sMsp).
    pub msp: Option<i16>,
    /// HP cost (C++: sHP).
    pub hp: Option<i16>,
    /// SP cost (C++: sSp).
    pub s_sp: Option<i16>,
    /// Required item group (C++: bItemGroup).
    pub item_group: Option<i16>,
    /// Required use item ID (C++: iUseItem).
    pub use_item: Option<i32>,
    /// Cast time in ticks (C++: bCastTime).
    pub cast_time: Option<i16>,
    /// Cooldown time (C++: sReCastTime).
    pub recast_time: Option<i16>,
    /// Base success rate (C++: bSuccessRate).
    pub success_rate: Option<i16>,
    /// Skill type category 1 — determines sub-table join (C++: bType[0]).
    pub type1: Option<i16>,
    /// Skill type category 2 — secondary type (C++: bType[1]).
    pub type2: Option<i16>,
    /// Skill range (C++: sRange). Column is quoted in SQL as it is reserved.
    pub range: Option<i16>,
    /// Extra parameter (C++: sEtc).
    pub etc: Option<i16>,
    /// Standing requirement (C++: sUseStanding).
    pub use_standing: Option<i16>,
    /// Skill check flags (C++: sSkillCheck).
    pub skill_check: Option<i16>,
    /// Ice/lightning rate modifier (C++: icelightrate).
    pub icelightrate: Option<i16>,
}

/// Melee attack skill parameters.
/// Keyed by `i_num`. Contains hit type, hit rate, combo system, and
/// per-target damage modifiers for physical melee skills.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType1Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Hit type (C++: bHitType).
    pub hit_type: Option<i32>,
    /// Hit rate modifier (C++: sHitRate).
    pub hit_rate: Option<i32>,
    /// Base hit value (C++: sHit).
    pub hit: Option<i32>,
    /// Additional flat damage (C++: sAddDamage).
    pub add_damage: Option<i32>,
    /// Combo type flag (C++: bComboType).
    pub combo_type: Option<i32>,
    /// Number of combo hits (C++: bComboCount).
    pub combo_count: Option<i32>,
    /// Damage per combo hit (C++: sComboDamage).
    pub combo_damage: Option<i32>,
    /// Attack range (C++: sRange). Column is quoted in SQL as it is reserved.
    pub range: Option<i32>,
    /// Delay between hits (C++: bDelay).
    pub delay: Option<i32>,
    /// Additional damage percent vs users (C++: iADPtoUser).
    pub add_dmg_perc_to_user: Option<i32>,
    /// Additional damage percent vs NPCs (C++: iADPtoNPC).
    pub add_dmg_perc_to_npc: Option<i32>,
}

/// Ranged/archery attack skill parameters.
/// Keyed by `i_num`. Contains range extensions and arrow requirements.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType2Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Hit type (C++: bHitType).
    pub hit_type: Option<i32>,
    /// Hit rate modifier (C++: sHitRate).
    pub hit_rate: Option<i32>,
    /// Additional flat damage (C++: sAddDamage).
    pub add_damage: Option<i32>,
    /// Additional range (C++: sAddRange).
    pub add_range: Option<i32>,
    /// Whether an arrow is consumed (C++: bNeedArrow).
    pub need_arrow: Option<i32>,
    /// Additional damage percent vs users (C++: iADPtoUser).
    pub add_dmg_perc_to_user: Option<i16>,
    /// Additional damage percent vs NPCs (C++: iADPtoNPC).
    pub add_dmg_perc_to_npc: Option<i16>,
}

/// DOT / direct magic damage skill parameters.
/// Keyed by `i_num`. Contains initial and periodic damage, duration,
/// elemental attribute, and area-of-effect radius.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType3Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Damage delivery type (C++: bDirectType).
    pub direct_type: Option<i32>,
    /// Initial damage on cast (C++: sFirstDamage).
    pub first_damage: Option<i32>,
    /// Periodic tick damage (C++: sTimeDamage).
    pub time_damage: Option<i32>,
    /// Effect duration in seconds (C++: bDuration).
    pub duration: Option<i32>,
    /// Elemental attribute (C++: bAttribute).
    pub attribute: Option<i32>,
    /// Area-of-effect radius (C++: bRadius).
    pub radius: Option<i32>,
    /// Additional damage percent vs users (C++: iADPtoUser).
    pub add_dmg_perc_to_user: Option<i16>,
    /// Additional damage percent vs NPCs (C++: iADPtoNPC).
    pub add_dmg_perc_to_npc: Option<i16>,
}

/// Buff/debuff skill parameters.
/// Keyed by `i_num`. The largest sub-table with extensive stat modifier
/// fields covering attack, defense, resistances, and stats.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType4Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Buff type flag (C++: bBuffType).
    pub buff_type: Option<i32>,
    /// Area-of-effect radius (C++: bRadius).
    pub radius: Option<i32>,
    /// Buff duration in seconds (C++: sDuration).
    pub duration: Option<i32>,
    /// Attack speed modifier (C++: bAttackSpeed).
    pub attack_speed: Option<i32>,
    /// Movement speed modifier (C++: bSpeed).
    pub speed: Option<i32>,
    /// Armor class modifier (C++: sAC).
    pub ac: Option<i32>,
    /// Armor class percent modifier (C++: sACPct).
    pub ac_pct: Option<i32>,
    /// Physical attack modifier (C++: bAttack).
    pub attack: Option<i32>,
    /// Magic attack modifier (C++: bMagicAttack).
    pub magic_attack: Option<i32>,
    /// Max HP modifier (C++: sMaxHP).
    pub max_hp: Option<i32>,
    /// Max HP percent modifier (C++: sMaxHPPct).
    pub max_hp_pct: Option<i32>,
    /// Max MP modifier (C++: sMaxMP).
    pub max_mp: Option<i32>,
    /// Max MP percent modifier (C++: sMaxMPPct).
    pub max_mp_pct: Option<i32>,
    /// Strength modifier (C++: bStr).
    pub str: Option<i32>,
    /// Stamina modifier (C++: bSta).
    pub sta: Option<i32>,
    /// Dexterity modifier (C++: bDex).
    pub dex: Option<i32>,
    /// Intelligence modifier (C++: bIntel).
    pub intel: Option<i32>,
    /// Charisma modifier (C++: bCha).
    pub cha: Option<i32>,
    /// Fire resistance modifier (C++: bFireR).
    pub fire_r: Option<i32>,
    /// Cold resistance modifier (C++: bColdR).
    pub cold_r: Option<i32>,
    /// Lightning resistance modifier (C++: bLightningR).
    pub lightning_r: Option<i32>,
    /// Magic resistance modifier (C++: bMagicR).
    pub magic_r: Option<i32>,
    /// Disease resistance modifier (C++: bDiseaseR).
    pub disease_r: Option<i32>,
    /// Poison resistance modifier (C++: bPoisonR).
    pub poison_r: Option<i32>,
    /// Experience percent modifier (C++: sExpPct).
    pub exp_pct: Option<i32>,
    /// Special amount value (C++: sSpecialAmount).
    pub special_amount: Option<i32>,
    /// Hit rate modifier (C++: bHitRate).
    pub hit_rate: Option<i32>,
    /// Evasion/avoid rate modifier (C++: sAvoidRate).
    pub avoid_rate: Option<i32>,
}

/// Resurrection/recovery skill parameters.
/// Keyed by `i_num`. Contains recovery type, experience recovery, and
/// material (stone) requirements.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType5Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Recovery/resurrection type (C++: bType). Column is quoted in SQL as it is reserved.
    pub r#type: Option<i32>,
    /// Experience recovery percent (C++: bExpRecover).
    pub exp_recover: Option<i32>,
    /// Required resurrection stone item ID (C++: sNeedStone).
    pub need_stone: Option<i32>,
}

/// Transformation skill parameters.
/// Keyed by `i_num`. Contains full stat overrides for the transformed state,
/// including HP, speed, attack, defense, and all elemental resistances.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType6Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Transformation name (not loaded by C++ runtime).
    pub name: Option<String>,
    /// Description text (not loaded by C++ runtime).
    pub description: Option<String>,
    /// Visual size multiplier (C++: sSize).
    pub size: i32,
    /// NPC/monster model ID to transform into (C++: sTransformID).
    pub transform_id: i32,
    /// Transform duration in seconds (C++: sDuration).
    pub duration: i32,
    /// Max HP while transformed (C++: sMaxHp).
    pub max_hp: i32,
    /// Max MP while transformed (C++: sMaxMp).
    pub max_mp: i32,
    /// Movement speed while transformed (C++: bSpeed).
    pub speed: i32,
    /// Attack speed while transformed (C++: sAttackSpeed).
    pub attack_speed: i32,
    /// Total hit/attack power (C++: sTotalHit).
    pub total_hit: i32,
    /// Total armor class (C++: sTotalAc).
    pub total_ac: i32,
    /// Total hit rate (C++: sTotalHitRate).
    pub total_hit_rate: i32,
    /// Total evasion rate (C++: sTotalEvasionRate).
    pub total_evasion_rate: i32,
    /// Fire resistance (C++: sTotalFireR).
    pub total_fire_r: i32,
    /// Cold resistance (C++: sTotalColdR).
    pub total_cold_r: i32,
    /// Lightning resistance (C++: sTotalLightningR).
    pub total_lightning_r: i32,
    /// Magic resistance (C++: sTotalMagicR).
    pub total_magic_r: i32,
    /// Disease resistance (C++: sTotalDiseaseR).
    pub total_disease_r: i32,
    /// Poison resistance (C++: sTotalPoisonR).
    pub total_poison_r: i32,
    /// Class restriction (C++: sClass).
    pub class: i32,
    /// Whether user can still use skills (C++: bUserSkillUse).
    pub user_skill_use: i32,
    /// Required item to transform (C++: bNeedItem).
    pub need_item: i32,
    /// Skill success rate (C++: bSkillSuccessRate).
    pub skill_success_rate: i16,
    /// Whether friendly to monsters (C++: bMonsterFriendly).
    pub monster_friendly: i32,
    /// Nation restriction (C++: bNation).
    pub nation: i32,
}

/// Summoning / crowd-control skill parameters (type 7).
/// Keyed by `n_index`. Contains summon/CC parameters including monster ID,
/// target changes, state changes, and duration.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType7Row {
    /// Magic number (C++: iNum). Column name differs from other tables.
    pub n_index: i32,
    /// Skill name (not loaded by C++ runtime).
    pub str_name: Option<String>,
    /// Skill note/description (not loaded by C++ runtime).
    pub str_note: Option<String>,
    /// Valid target group (C++: bValidGroup).
    pub valid_group: i16,
    /// Nation change flag (C++: bNationChange).
    pub nation_change: i16,
    /// Monster template ID to summon (C++: sMonsterNum).
    pub monster_num: i16,
    /// Target change type (C++: bTargetChange).
    pub target_change: i16,
    /// State change effect (C++: bStateChange).
    pub state_change: i16,
    /// Area-of-effect radius (C++: bRadius).
    pub radius: i16,
    /// Hit rate modifier (C++: sHitRate).
    pub hit_rate: i16,
    /// Effect duration (C++: sDuration).
    pub duration: i16,
    /// Damage dealt (C++: sDamage).
    pub damage: i16,
    /// Vision range modifier (C++: bVision).
    pub vision: i16,
    /// Required item ID (C++: nNeedItem).
    pub need_item: i32,
}

/// Teleportation / warp skill parameters.
/// Keyed by `i_num`. Contains warp type, radius, experience recovery,
/// and kick-back distance.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType8Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Skill name (not loaded by C++ runtime).
    pub name: Option<String>,
    /// Description text (not loaded by C++ runtime).
    pub description: Option<String>,
    /// Target type (C++: bTarget).
    pub target: i16,
    /// Effect radius (C++: sRadius).
    pub radius: i16,
    /// Warp behaviour type (C++: bWarpType).
    pub warp_type: i16,
    /// Experience recovery on use (C++: sExpRecover).
    pub exp_recover: i16,
    /// Knockback distance (C++: sKickDistance).
    pub kick_distance: i16,
}

/// Advanced debuff / crowd-control skill parameters (type 9).
/// Keyed by `i_num`. Similar to type 7 but with wider integer fields
/// for radius, vision, and damage.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MagicType9Row {
    /// Magic number (foreign key to `magic.magic_num`).
    pub i_num: i32,
    /// Skill name (not loaded by C++ runtime).
    pub name: Option<String>,
    /// Description text (not loaded by C++ runtime).
    pub description: Option<String>,
    /// Valid target group (C++: bValidGroup).
    pub valid_group: Option<i16>,
    /// Nation change flag (C++: bNationChange).
    pub nation_change: Option<i16>,
    /// Monster template ID (C++: sMonsterNum).
    pub monster_num: Option<i32>,
    /// Target change type (C++: bTargetChange).
    pub target_change: Option<i16>,
    /// State change effect (C++: bStateChange).
    pub state_change: Option<i16>,
    /// Area-of-effect radius (C++: sRadius).
    pub radius: Option<i16>,
    /// Hit rate modifier (C++: sHitRate).
    pub hit_rate: Option<i16>,
    /// Effect duration (C++: sDuration).
    pub duration: Option<i32>,
    /// Damage dealt (C++: sDamage).
    pub add_damage: Option<i16>,
    /// Vision range modifier (C++: sVision).
    pub vision: Option<i16>,
    /// Required item ID (C++: nNeedItem).
    pub need_item: Option<i16>,
}
