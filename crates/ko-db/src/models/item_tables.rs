//! Item-related reference table models — maps to PostgreSQL tables.
//!
//! C++ Reference:
//! - `shared/database/ItemTableSet.h` — _ITEM_OP, _SET_ITEM, _ITEM_EXCHANGE
//! - `shared/database/MagicTableSet.h` — _K_MONSTER_ITEM
//! - `shared/database/NpcItemSet.h` — _K_NPC_ITEM
//! - `shared/database/ItemUpgradeSet.h` — _ITEM_UPGRADE
//! - `shared/database/MakeWeaponTableSet.h` — _MAKE_WEAPON
//! - `shared/database/MakeDefensiveTableSet.h` — _MAKE_WEAPON (reused)
//! - `shared/database/MakeGradeItemTableSet.h` — _MAKE_ITEM_GRADE_CODE
//! - `shared/database/MakeLareItemTableSet.h` — _MAKE_ITEM_LARE_CODE
//! - `shared/database/MakeItemGroupSet.h` — _MAKE_ITEM_GROUP, _MAKE_ITEM_GROUP_RANDOM
//! - `shared/database/RentalItemSet.h` — _RENTAL_ITEM
//!
//! These tables are bulk-loaded at startup and cached in-memory.

/// Item special effect entry from the `item_op` table.
///
/// Maps triggered skill effects to items (e.g., weapon procs).
/// MSSQL source: `ITEM_OP` (2,703 rows).
/// C++ equivalent: `_ITEM_OP` (ItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemOpRow {
    /// Item ID that has the special effect.
    pub item_id: i32,
    /// Trigger condition type (e.g., on-hit, on-defense, passive).
    pub trigger_type: i16,
    /// Skill ID that gets triggered.
    pub skill_id: i32,
    /// Trigger chance (0-100 or per-10000 depending on type).
    pub trigger_rate: i16,
}

/// Set item bonus entry from the `set_item` table.
///
/// Defines bonuses granted when wearing a complete item set.
/// MSSQL source: `SET_ITEM` (1,165 rows).
/// C++ equivalent: `_SET_ITEM` (ItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SetItemRow {
    /// Set identifier (matches item's set_id field).
    pub set_index: i32,
    /// Display name for the set.
    pub set_name: Option<String>,
    /// Armor class bonus.
    pub ac_bonus: i16,
    /// Max HP bonus.
    pub hp_bonus: i16,
    /// Max MP bonus.
    pub mp_bonus: i16,
    /// Strength bonus.
    pub strength_bonus: i16,
    /// Stamina bonus.
    pub stamina_bonus: i16,
    /// Dexterity bonus.
    pub dexterity_bonus: i16,
    /// Intelligence bonus.
    pub intel_bonus: i16,
    /// Charisma bonus.
    pub charisma_bonus: i16,
    /// Flame resistance bonus.
    pub flame_resistance: i16,
    /// Glacier resistance bonus.
    pub glacier_resistance: i16,
    /// Lightning resistance bonus.
    pub lightning_resistance: i16,
    /// Poison resistance bonus.
    pub poison_resistance: i16,
    /// Magic resistance bonus.
    pub magic_resistance: i16,
    /// Curse resistance bonus.
    pub curse_resistance: i16,
    /// XP bonus percent.
    pub xp_bonus_percent: i16,
    /// Gold/coin bonus percent.
    pub coin_bonus_percent: i16,
    /// Attack power bonus percent.
    pub ap_bonus_percent: i16,
    /// Attack power bonus class restriction type.
    pub ap_bonus_class_type: i16,
    /// Attack power bonus class percent.
    pub ap_bonus_class_percent: i16,
    /// Armor class bonus class restriction type.
    pub ac_bonus_class_type: i16,
    /// Armor class bonus class percent.
    pub ac_bonus_class_percent: i16,
    /// Max weight bonus.
    pub max_weight_bonus: i16,
    /// Nation point bonus.
    pub np_bonus: i16,
    /// Unknown field 1.
    pub unk1: i16,
    /// Unknown field 2.
    pub unk2: i16,
    /// Unknown field 3.
    pub unk3: i16,
    /// Unknown field 4.
    pub unk4: i16,
    /// Unknown field 5.
    pub unk5: i16,
    /// Unknown field 6.
    pub unk6: i16,
    /// Unknown field 7.
    pub unk7: i16,
    /// Unknown field 8.
    pub unk8: i16,
    /// Unknown field 9.
    pub unk9: i16,
    /// Unknown field 10.
    pub unk10: i16,
    /// Unknown field 11.
    pub unk11: i16,
    /// Unknown field 12.
    pub unk12: i16,
    /// Unknown field 13.
    pub unk13: i16,
    /// Unknown field 14.
    pub unk14: i16,
    /// Unknown field 15 (TBL col_39).
    pub unk15: i16,
}

/// Monster drop table entry from the `monster_item` table.
///
/// Each monster has up to 12 item drop slots with associated drop rates.
/// MSSQL source: `K_MONSTER_ITEM` (2,154 rows).
/// C++ equivalent: `_K_MONSTER_ITEM` (MagicTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MonsterItemRow {
    /// Monster drop table index (referenced by npc_template.item_table).
    pub s_index: i16,
    /// Item slot 1 ID.
    pub item01: i32,
    /// Item slot 1 drop rate (per-10000).
    pub percent01: i16,
    /// Item slot 2 ID.
    pub item02: i32,
    /// Item slot 2 drop rate.
    pub percent02: i16,
    /// Item slot 3 ID.
    pub item03: i32,
    /// Item slot 3 drop rate.
    pub percent03: i16,
    /// Item slot 4 ID.
    pub item04: i32,
    /// Item slot 4 drop rate.
    pub percent04: i16,
    /// Item slot 5 ID.
    pub item05: i32,
    /// Item slot 5 drop rate.
    pub percent05: i16,
    /// Item slot 6 ID.
    pub item06: i32,
    /// Item slot 6 drop rate.
    pub percent06: i16,
    /// Item slot 7 ID.
    pub item07: i32,
    /// Item slot 7 drop rate.
    pub percent07: i16,
    /// Item slot 8 ID.
    pub item08: i32,
    /// Item slot 8 drop rate.
    pub percent08: i16,
    /// Item slot 9 ID.
    pub item09: i32,
    /// Item slot 9 drop rate.
    pub percent09: i16,
    /// Item slot 10 ID.
    pub item10: i32,
    /// Item slot 10 drop rate.
    pub percent10: i16,
    /// Item slot 11 ID.
    pub item11: i32,
    /// Item slot 11 drop rate.
    pub percent11: i16,
    /// Item slot 12 ID.
    pub item12: i32,
    /// Item slot 12 drop rate.
    pub percent12: i16,
}

/// Item exchange/crafting recipe from the `item_exchange` table.
///
/// Defines crafting recipes: up to 5 input items produce up to 5 output items.
/// MSSQL source: `ITEM_EXCHANGE` (5,023 rows).
/// C++ equivalent: `_ITEM_EXCHANGE` (ItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemExchangeRow {
    /// Exchange recipe index.
    pub n_index: i32,
    /// Random flag (0=fixed, 1=random selection, 2=random with rate).
    pub random_flag: i16,
    /// Origin (input) item 1 ID.
    pub origin_item_num1: i32,
    /// Origin item 1 required count.
    pub origin_item_count1: i32,
    /// Origin item 2 ID.
    pub origin_item_num2: i32,
    /// Origin item 2 required count.
    pub origin_item_count2: i32,
    /// Origin item 3 ID.
    pub origin_item_num3: i32,
    /// Origin item 3 required count.
    pub origin_item_count3: i32,
    /// Origin item 4 ID.
    pub origin_item_num4: i32,
    /// Origin item 4 required count.
    pub origin_item_count4: i32,
    /// Origin item 5 ID.
    pub origin_item_num5: i32,
    /// Origin item 5 required count.
    pub origin_item_count5: i32,
    /// Exchange (output) item 1 ID.
    pub exchange_item_num1: i32,
    /// Exchange item 1 output count.
    pub exchange_item_count1: i32,
    /// Exchange item 2 ID.
    pub exchange_item_num2: i32,
    /// Exchange item 2 output count.
    pub exchange_item_count2: i32,
    /// Exchange item 3 ID.
    pub exchange_item_num3: i32,
    /// Exchange item 3 output count.
    pub exchange_item_count3: i32,
    /// Exchange item 4 ID.
    pub exchange_item_num4: i32,
    /// Exchange item 4 output count.
    pub exchange_item_count4: i32,
    /// Exchange item 5 ID.
    pub exchange_item_num5: i32,
    /// Exchange item 5 output count.
    pub exchange_item_count5: i32,
    /// Exchange item 1 duration (seconds, 0 = permanent).
    pub exchange_item_time1: i32,
    /// Exchange item 2 duration.
    pub exchange_item_time2: i32,
    /// Exchange item 3 duration.
    pub exchange_item_time3: i32,
    /// Exchange item 4 duration.
    pub exchange_item_time4: i32,
    /// Exchange item 5 duration.
    pub exchange_item_time5: i32,
}

/// Item upgrade settings entry from the `item_upgrade_settings` table.
///
/// Defines the required materials, success rates, and costs for item upgrades.
/// MSSQL source: `ITEM_UPGRADE_SETTINGS` (257 rows).
/// C++ equivalent: Used by `CGameServerDlg::LoadItemUpgradeTable()`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemUpgradeSettingsRow {
    /// Required material item 1 ID.
    pub req_item_id1: i32,
    /// Required material item 1 name (display only).
    pub req_item_name1: Option<String>,
    /// Required material item 2 ID.
    pub req_item_id2: i32,
    /// Required material item 2 name (display only).
    pub req_item_name2: Option<String>,
    /// Upgrade description note.
    pub upgrade_note: Option<String>,
    /// Item type category for this upgrade rule.
    pub item_type: i16,
    /// Item rate/class within the type.
    pub item_rate: i16,
    /// Item grade (upgrade level target, e.g., +1 through +10).
    pub item_grade: i16,
    /// Required coins (gold) for the upgrade attempt.
    pub item_req_coins: i32,
    /// Success rate (out of 10,000).
    pub success_rate: i16,
}

/// New upgrade recipe from the `new_upgrade` table.
///
/// Defines specific item-to-item upgrade transformations (e.g., +5 sword -> +6 sword).
/// MSSQL source: `NEW_UPGRADE1` (18,148 rows).
/// C++ equivalent: `_NEW_UPGRADE` (ItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewUpgradeRow {
    /// Upgrade recipe index.
    pub n_index: i32,
    /// Description of the origin item.
    pub str_note: Option<String>,
    /// Origin item number (the item being upgraded).
    pub origin_number: i32,
    /// Description of the resulting item.
    pub n_str_note: Option<String>,
    /// New item number after successful upgrade.
    pub new_number: i32,
    /// Required material item ID for this upgrade.
    pub req_item: i32,
    /// Grade/level of this upgrade.
    pub grade: i16,
}

/// NPC drop table entry from the `npc_item` table.
///
/// Each NPC has up to 12 item drop slots with associated drop rates.
/// MSSQL source: `K_NPC_ITEM`.
/// C++ equivalent: `_K_NPC_ITEM` (NpcItemSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NpcItemRow {
    /// NPC drop table index.
    pub s_index: i16,
    /// Item slot 1 ID.
    pub item01: i32,
    /// Item slot 1 drop rate (per-10000).
    pub percent01: i16,
    /// Item slot 2 ID.
    pub item02: i32,
    /// Item slot 2 drop rate.
    pub percent02: i16,
    /// Item slot 3 ID.
    pub item03: i32,
    /// Item slot 3 drop rate.
    pub percent03: i16,
    /// Item slot 4 ID.
    pub item04: i32,
    /// Item slot 4 drop rate.
    pub percent04: i16,
    /// Item slot 5 ID.
    pub item05: i32,
    /// Item slot 5 drop rate.
    pub percent05: i16,
    /// Item slot 6 ID.
    pub item06: i32,
    /// Item slot 6 drop rate.
    pub percent06: i16,
    /// Item slot 7 ID.
    pub item07: i32,
    /// Item slot 7 drop rate.
    pub percent07: i16,
    /// Item slot 8 ID.
    pub item08: i32,
    /// Item slot 8 drop rate.
    pub percent08: i16,
    /// Item slot 9 ID.
    pub item09: i32,
    /// Item slot 9 drop rate.
    pub percent09: i16,
    /// Item slot 10 ID.
    pub item10: i32,
    /// Item slot 10 drop rate.
    pub percent10: i16,
    /// Item slot 11 ID.
    pub item11: i32,
    /// Item slot 11 drop rate.
    pub percent11: i16,
    /// Item slot 12 ID.
    pub item12: i32,
    /// Item slot 12 drop rate.
    pub percent12: i16,
}

/// Item upgrade recipe from the `item_upgrade` table.
///
/// Defines NPC-based upgrade recipes with required materials and success rates.
/// MSSQL source: `ITEM_UPGRADE`.
/// C++ equivalent: `_ITEM_UPGRADE` (ItemUpgradeSet.h).
/// Constants: `MAX_ITEMS_REQ_FOR_UPGRADE = 8`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemUpgradeRow {
    /// Upgrade recipe index.
    pub n_index: i32,
    /// NPC number that performs this upgrade.
    pub npc_num: Option<i16>,
    /// Origin item type category.
    pub origin_type: Option<i16>,
    /// Origin item ID being upgraded.
    pub origin_item: Option<i32>,
    /// Required material item 1.
    pub req_item1: Option<i32>,
    /// Required material item 2.
    pub req_item2: Option<i32>,
    /// Required material item 3.
    pub req_item3: Option<i32>,
    /// Required material item 4.
    pub req_item4: Option<i32>,
    /// Required material item 5.
    pub req_item5: Option<i32>,
    /// Required material item 6.
    pub req_item6: Option<i32>,
    /// Required material item 7.
    pub req_item7: Option<i32>,
    /// Required material item 8.
    pub req_item8: Option<i32>,
    /// Required gold (Noah) for the upgrade.
    pub req_noah: Option<i32>,
    /// Rate type (determines which rate column to use).
    pub rate_type: Option<i16>,
    /// General success rate.
    pub gen_rate: Option<i16>,
    /// Trina scroll success rate.
    pub trina_rate: Option<i16>,
    /// Karivdis scroll success rate.
    pub karivdis_rate: Option<i16>,
    /// Resulting item ID on success.
    pub give_item: Option<i32>,
}

/// Weapon crafting template from the `make_weapon` table.
///
/// Maps crafting level to item IDs for 12 weapon classes.
/// MSSQL source: `MAKE_WEAPON`.
/// C++ equivalent: `_MAKE_WEAPON` (MakeWeaponTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeWeaponRow {
    /// Crafting level index.
    pub by_level: i16,
    /// Weapon class 1 item ID.
    pub class_1: i16,
    /// Weapon class 2 item ID.
    pub class_2: i16,
    /// Weapon class 3 item ID.
    pub class_3: i16,
    /// Weapon class 4 item ID.
    pub class_4: i16,
    /// Weapon class 5 item ID.
    pub class_5: i16,
    /// Weapon class 6 item ID.
    pub class_6: i16,
    /// Weapon class 7 item ID.
    pub class_7: i16,
    /// Weapon class 8 item ID.
    pub class_8: i16,
    /// Weapon class 9 item ID.
    pub class_9: i16,
    /// Weapon class 10 item ID.
    pub class_10: i16,
    /// Weapon class 11 item ID.
    pub class_11: i16,
    /// Weapon class 12 item ID.
    pub class_12: i16,
}

/// Defensive crafting template from the `make_defensive` table.
///
/// Maps crafting level to item IDs for 7 defensive classes.
/// MSSQL source: `MAKE_DEFENSIVE`.
/// C++ equivalent: `_MAKE_WEAPON` reused (MakeDefensiveTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeDefensiveRow {
    /// Crafting level index.
    pub by_level: i16,
    /// Defensive class 1 item ID.
    pub class_1: i16,
    /// Defensive class 2 item ID.
    pub class_2: i16,
    /// Defensive class 3 item ID.
    pub class_3: i16,
    /// Defensive class 4 item ID.
    pub class_4: i16,
    /// Defensive class 5 item ID.
    pub class_5: i16,
    /// Defensive class 6 item ID.
    pub class_6: i16,
    /// Defensive class 7 item ID.
    pub class_7: i16,
}

/// Crafting grade code from the `make_item_gradecode` table.
///
/// Maps item index to 9 grade probability values.
/// MSSQL source: `MAKE_ITEM_GRADECODE`.
/// C++ equivalent: `_MAKE_ITEM_GRADE_CODE` (MakeGradeItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeItemGradeCodeRow {
    /// Item crafting category index.
    pub item_index: i16,
    /// Grade 1 weight/probability.
    pub grade_1: i16,
    /// Grade 2 weight/probability.
    pub grade_2: i16,
    /// Grade 3 weight/probability.
    pub grade_3: i16,
    /// Grade 4 weight/probability.
    pub grade_4: i16,
    /// Grade 5 weight/probability.
    pub grade_5: i16,
    /// Grade 6 weight/probability.
    pub grade_6: i16,
    /// Grade 7 weight/probability.
    pub grade_7: i16,
    /// Grade 8 weight/probability.
    pub grade_8: i16,
    /// Grade 9 weight/probability.
    pub grade_9: i16,
}

/// Crafting rarity code from the `make_item_larecode` table.
///
/// Maps level grade to rarity type probabilities.
/// MSSQL source: `MAKE_ITEM_LARECODE`.
/// C++ equivalent: `_MAKE_ITEM_LARE_CODE` (MakeLareItemTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeItemLareCodeRow {
    /// Level grade index.
    pub level_grade: i16,
    /// Rare (Lare) item probability.
    pub lare_item: i16,
    /// Magic item probability.
    pub magic_item: i16,
    /// General item probability.
    pub general_item: i16,
}

/// Crafting item group from the `make_item_group` table.
///
/// Groups up to 200 possible output items for crafting.
/// MSSQL source: `MAKE_ITEM_GROUP`.
/// C++ equivalent: `_MAKE_ITEM_GROUP` (MakeItemGroupSet.h).
/// Note: Normalized from 200-column layout to a PostgreSQL integer array.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeItemGroupRow {
    /// Item group number.
    pub group_num: i32,
    /// List of item IDs in this group.
    pub items: Vec<i32>,
}

/// Random crafting item group mapping from the `make_item_group_random` table.
///
/// Maps indices to item IDs and group numbers for random crafting selection.
/// MSSQL source: `MAKE_ITEM_GROUP_RANDOM`.
/// C++ equivalent: `_MAKE_ITEM_GROUP_RANDOM` (MakeItemGroupSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeItemGroupRandomRow {
    /// Record index.
    pub n_index: i32,
    /// Item ID.
    pub item_id: i32,
    /// Group number this item belongs to.
    pub group_no: i32,
}

/// Crafting item code lookup from the `make_item` table.
///
/// Maps a weighted sIndex (1..10000) to an (item_code, item_level) pair.
/// Used by the loot system (ItemProdution) to generate random item drops.
/// MSSQL source: `MAKE_ITEM` (10,000 rows).
/// C++ Reference: `CNpc::ItemProdution()` in Npc.cpp.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MakeItemRow {
    /// Weighted recipe/lookup index (1..10000).
    pub s_index: i16,
    /// Output item code.
    pub item_code: i32,
    /// Output item level.
    pub item_level: i16,
}

/// Rental item entry from the `rental_item` table.
///
/// Defines items available for player-to-player rental.
/// MSSQL source: `RENTAL_ITEM`.
/// C++ equivalent: `_RENTAL_ITEM` (RentalItemSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct RentalItemRow {
    /// Rental record index.
    pub rental_index: i32,
    /// Item ID being rented.
    pub item_index: i32,
    /// Item durability.
    pub durability: i16,
    /// Item serial number.
    pub serial_number: i64,
    /// Registration type.
    pub reg_type: i16,
    /// Item type category.
    pub item_type: i16,
    /// Item class.
    pub item_class: i16,
    /// Rental time (hours or minutes depending on type).
    pub rental_time: i16,
    /// Rental cost (gold).
    pub rental_money: i32,
    /// Lender character ID.
    pub lender_char_id: String,
    /// Borrower character ID.
    pub borrower_char_id: String,
}

/// NPC sell table entry from the `item_sell_table` table.
///
/// Each row maps a selling group (NPC shop) to 24 item slots.
/// Used at buy time to validate that the requested item is actually
/// sold by the NPC the player is interacting with.
///
/// MSSQL source: `ITEM_SELLTABLE` (457 rows, 42 selling groups).
/// C++ equivalent: `_ITEM_SELLTABLE` (ItemSellTableSet.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemSellTableRow {
    /// Row index (primary key in original DB).
    pub n_index: i32,
    /// Selling group ID — matches `npc_template.i_selling_group`.
    pub i_selling_group: i32,
    /// Item slot 1 ID.
    pub item1: i32,
    /// Item slot 2 ID.
    pub item2: i32,
    /// Item slot 3 ID.
    pub item3: i32,
    /// Item slot 4 ID.
    pub item4: i32,
    /// Item slot 5 ID.
    pub item5: i32,
    /// Item slot 6 ID.
    pub item6: i32,
    /// Item slot 7 ID.
    pub item7: i32,
    /// Item slot 8 ID.
    pub item8: i32,
    /// Item slot 9 ID.
    pub item9: i32,
    /// Item slot 10 ID.
    pub item10: i32,
    /// Item slot 11 ID.
    pub item11: i32,
    /// Item slot 12 ID.
    pub item12: i32,
    /// Item slot 13 ID.
    pub item13: i32,
    /// Item slot 14 ID.
    pub item14: i32,
    /// Item slot 15 ID.
    pub item15: i32,
    /// Item slot 16 ID.
    pub item16: i32,
    /// Item slot 17 ID.
    pub item17: i32,
    /// Item slot 18 ID.
    pub item18: i32,
    /// Item slot 19 ID.
    pub item19: i32,
    /// Item slot 20 ID.
    pub item20: i32,
    /// Item slot 21 ID.
    pub item21: i32,
    /// Item slot 22 ID.
    pub item22: i32,
    /// Item slot 23 ID.
    pub item23: i32,
    /// Item slot 24 ID.
    pub item24: i32,
}

impl ItemSellTableRow {
    /// Get the item ID at a specific slot index (0-23).
    ///
    /// Returns 0 for out-of-range indices (matching C++ behavior).
    pub fn item_at(&self, index: usize) -> i32 {
        match index {
            0 => self.item1,
            1 => self.item2,
            2 => self.item3,
            3 => self.item4,
            4 => self.item5,
            5 => self.item6,
            6 => self.item7,
            7 => self.item8,
            8 => self.item9,
            9 => self.item10,
            10 => self.item11,
            11 => self.item12,
            12 => self.item13,
            13 => self.item14,
            14 => self.item15,
            15 => self.item16,
            16 => self.item17,
            17 => self.item18,
            18 => self.item19,
            19 => self.item20,
            20 => self.item21,
            21 => self.item22,
            22 => self.item23,
            23 => self.item24,
            _ => 0,
        }
    }
}

/// Crafting recipe entry from the `item_special_sewing` table.
///
/// Each recipe requires up to 10 material items and produces one output item.
/// Used by the Shozin Exchange (Special Part Sewing) crafting system.
/// MSSQL source: `ITEM_SPECIAL_SEWING` (2,468 rows).
/// C++ equivalent: `SPECIAL_PART_SEWING_EXCHANGE` (GameDefine.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemSpecialSewingRow {
    /// Recipe index (primary key).
    pub n_index: i32,
    /// Description of the recipe.
    pub description: Option<String>,
    /// Required material item 1 ID.
    pub req_item_id_1: i32,
    /// Required material item 1 count.
    pub req_item_count_1: i32,
    /// Required material item 2 ID.
    pub req_item_id_2: i32,
    /// Required material item 2 count.
    pub req_item_count_2: i32,
    /// Required material item 3 ID.
    pub req_item_id_3: i32,
    /// Required material item 3 count.
    pub req_item_count_3: i32,
    /// Required material item 4 ID.
    pub req_item_id_4: i32,
    /// Required material item 4 count.
    pub req_item_count_4: i32,
    /// Required material item 5 ID.
    pub req_item_id_5: i32,
    /// Required material item 5 count.
    pub req_item_count_5: i32,
    /// Required material item 6 ID.
    pub req_item_id_6: i32,
    /// Required material item 6 count.
    pub req_item_count_6: i32,
    /// Required material item 7 ID.
    pub req_item_id_7: i32,
    /// Required material item 7 count.
    pub req_item_count_7: i32,
    /// Required material item 8 ID.
    pub req_item_id_8: i32,
    /// Required material item 8 count.
    pub req_item_count_8: i32,
    /// Required material item 9 ID.
    pub req_item_id_9: i32,
    /// Required material item 9 count.
    pub req_item_count_9: i32,
    /// Required material item 10 ID.
    pub req_item_id_10: i32,
    /// Required material item 10 count.
    pub req_item_count_10: i32,
    /// Resulting item ID on success.
    pub give_item_id: i32,
    /// Resulting item count on success.
    pub give_item_count: i32,
    /// Success rate (out of 10,000).
    pub success_rate: i32,
    /// NPC ID that performs this crafting (19073=Craftsman, 31402=Jeweler, 31510=type500).
    pub npc_id: i16,
    /// Whether to broadcast a notice on success.
    pub is_notice: bool,
    /// Whether Shadow Piece guarantees success for this recipe.
    pub is_shadow_success: bool,
}

/// Max number of material slots per crafting recipe (C++ `ITEMS_SPECIAL_EXCHANGE_GROUP`).
pub const ITEMS_SPECIAL_EXCHANGE_GROUP: usize = 10;

impl ItemSpecialSewingRow {
    /// Get the required item ID at a slot index (0-9).
    pub fn req_item_id_at(&self, index: usize) -> i32 {
        match index {
            0 => self.req_item_id_1,
            1 => self.req_item_id_2,
            2 => self.req_item_id_3,
            3 => self.req_item_id_4,
            4 => self.req_item_id_5,
            5 => self.req_item_id_6,
            6 => self.req_item_id_7,
            7 => self.req_item_id_8,
            8 => self.req_item_id_9,
            9 => self.req_item_id_10,
            _ => 0,
        }
    }

    /// Get the required item count at a slot index (0-9).
    pub fn req_item_count_at(&self, index: usize) -> i32 {
        match index {
            0 => self.req_item_count_1,
            1 => self.req_item_count_2,
            2 => self.req_item_count_3,
            3 => self.req_item_count_4,
            4 => self.req_item_count_5,
            5 => self.req_item_count_6,
            6 => self.req_item_count_7,
            7 => self.req_item_count_8,
            8 => self.req_item_count_9,
            9 => self.req_item_count_10,
            _ => 0,
        }
    }

    /// Count how many non-zero material slots this recipe uses.
    pub fn material_count(&self) -> u8 {
        let mut count = 0u8;
        for i in 0..ITEMS_SPECIAL_EXCHANGE_GROUP {
            if self.req_item_id_at(i) != 0 {
                count += 1;
            }
        }
        count
    }
}

/// Item smash entry from the `item_smash` table.
///
/// Used by the Old Man Exchange (Item Disassemble) system.
/// Each row defines a possible output item with a weighted rate.
/// MSSQL source: `ITEM_SMASH` (205 rows).
/// C++ equivalent: `_ITEM_EXCHANGE_CRASH` (GameDefine.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemSmashRow {
    /// Index (1000000-1999999=shields, 2000000-2999999=weapons, 3000000-3999999=armor,
    /// 4000000-4999999=earrings, 5000000-5999999=necklaces/rings).
    pub n_index: i32,
    /// Output item ID.
    pub item_id: i32,
    /// Output item count.
    pub count: i16,
    /// Drop rate weight (divided by 5 to fill the weighted array).
    pub rate: i16,
}

/// Special stone definition from the `k_special_stone` table.
///
/// Defines chaos stone summon configurations per zone.
/// MSSQL source: `K_SPECIAL_STONE` (18 rows).
/// C++ equivalent: `_K_SPECIAL_STONE` (GameDefine.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpecialStoneRow {
    /// Unique index.
    pub n_index: i32,
    /// Zone where this stone can appear.
    pub zone_id: i16,
    /// Main NPC ID (the chaos stone itself).
    pub main_npc: i32,
    /// Display name of the summon monster.
    pub monster_name: String,
    /// NPC ID to summon when stone is destroyed.
    pub summon_npc: i32,
    /// Number of NPCs to summon.
    pub summon_count: i32,
    /// Active status (1=active).
    pub status: i16,
}

/// Random item entry from the `item_random` table.
///
/// Used by event systems to generate random reward items.
/// MSSQL source: `ITEM_RANDOM` (58 rows).
/// C++ equivalent: `_ITEM_RANDOM` (GameDefine.h).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemRandomRow {
    /// Unique index.
    pub n_index: i32,
    /// Display name of the item.
    pub str_item_name: String,
    /// Item ID to give.
    pub item_id: i32,
    /// Quantity to give.
    pub item_count: i32,
    /// Rental duration (0 = permanent).
    pub rental_time: i32,
    /// Session/group ID for reward grouping.
    pub session_id: i16,
    /// Active status (1=active).
    pub status: i16,
}

/// Item group entry from the `item_group` table.
///
/// Groups items for random selection (e.g., Gavolt rewards).
/// MSSQL source: `ITEM_GROUP` (4 rows, 30 item columns normalized to array).
/// C++ equivalent: `_ITEM_GROUP`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemGroupRow {
    /// Group identifier.
    pub group_id: i16,
    /// Display name.
    pub name: Option<String>,
    /// Item IDs in this group (up to 30).
    pub items: Vec<i32>,
}

/// Item exchange experience entry from the `item_exchange_exp` table.
///
/// Defines exchange rewards with level-based output items.
/// MSSQL source: `ITEM_EXCHANGE_EXP` (204 rows).
/// C++ equivalent: `_ITEM_EXCHANGE_EXP`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemExchangeExpRow {
    /// Exchange index.
    pub n_index: i32,
    /// Random flag (0=fixed, >0 = random group).
    pub random_flag: Option<i16>,
    /// Exchange output item 1.
    pub exchange_item_num1: Option<i32>,
    /// Exchange output count 1.
    pub exchange_item_count1: Option<i32>,
    /// Exchange output item 2.
    pub exchange_item_num2: Option<i32>,
    /// Exchange output count 2.
    pub exchange_item_count2: Option<i32>,
    /// Exchange output item 3.
    pub exchange_item_num3: Option<i32>,
    /// Exchange output count 3.
    pub exchange_item_count3: Option<i32>,
    /// Exchange output item 4.
    pub exchange_item_num4: Option<i32>,
    /// Exchange output count 4.
    pub exchange_item_count4: Option<i32>,
    /// Exchange output item 5.
    pub exchange_item_num5: Option<i32>,
    /// Exchange output count 5.
    pub exchange_item_count5: Option<i32>,
    /// Expiration time for item 1.
    pub exchange_item_time1: Option<i32>,
    /// Expiration time for item 2.
    pub exchange_item_time2: Option<i32>,
    /// Expiration time for item 3.
    pub exchange_item_time3: Option<i32>,
    /// Expiration time for item 4.
    pub exchange_item_time4: Option<i32>,
    /// Expiration time for item 5.
    pub exchange_item_time5: Option<i32>,
}

/// Item give exchange entry from the `item_give_exchange` table.
///
/// Defines rob -> give item exchange rules.
/// MSSQL source: `ITEM_GIVE_EXCHANGE` (661 rows, 126 columns normalized to arrays).
/// C++ equivalent: `_ITEM_GIVE_EXCHANGE`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemGiveExchangeRow {
    /// Exchange index.
    pub exchange_index: i32,
    /// Item IDs to take from the player (up to 25).
    pub rob_item_ids: Vec<i32>,
    /// Counts to take from the player (up to 25).
    pub rob_item_counts: Vec<i32>,
    /// Item IDs to give to the player (up to 25).
    pub give_item_ids: Vec<i32>,
    /// Counts to give to the player (up to 25).
    pub give_item_counts: Vec<i32>,
    /// Expiration times for given items (up to 25).
    pub give_item_times: Vec<i32>,
}

/// Right-click exchange mapping from the `item_right_click_exchange` table.
///
/// Maps an item ID to its right-click exchange opcode.
/// MSSQL source: `ITEM_RIGHT_CLICK_EXCHANGE` (96 rows).
/// C++ equivalent: `_ITEM_RIGHT_CLICK_EXCHANGE`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemRightClickExchangeRow {
    /// Item ID that supports right-click exchange.
    pub item_id: i32,
    /// Exchange opcode (1=type1, 2=type2).
    pub opcode: i16,
}

/// Right exchange entry from the `item_right_exchange` table.
///
/// Defines right-click exchange reward tables.
/// MSSQL source: `ITEM_RIGHT_EXCHANGE` (66 rows, 80 columns normalized to arrays).
/// C++ equivalent: `_ITEM_RIGHT_EXCHANGE`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ItemRightExchangeRow {
    /// Source item ID.
    pub item_id: i32,
    /// Display name.
    pub str_name: Option<String>,
    /// Exchange type (1=selectable, 2=all).
    pub exchange_type: Option<i16>,
    /// Description.
    pub description: Option<String>,
    /// Number of exchange options.
    pub exchange_count: Option<i32>,
    /// Reward item IDs (up to 25).
    pub exchange_items: Vec<i32>,
    /// Reward item counts (up to 25).
    pub exchange_counts: Vec<i32>,
    /// Expiration times for reward items (up to 25).
    pub expiration_times: Vec<i32>,
}

/// Mining exchange entry from the `mining_exchange` table.
///
/// Defines ore-to-item crafting via mining NPC (Pitman 31511).
/// MSSQL source: `MINING_EXCHANGE` (0 rows in 25xx backup — schema only).
/// C++ equivalent: `_MINING_EXCHANGE` (GameDefine.h:2614-2625).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MiningExchangeRow {
    /// Record index (primary key).
    pub n_index: i16,
    /// NPC ID that performs the exchange (typically 31511).
    pub s_npc_id: i16,
    /// Whether to send a success effect (0/1).
    pub give_effect: i16,
    /// Ore classification (1 or 2).
    pub ore_type: i16,
    /// Item ID to consume (origin ore).
    pub n_origin_item_num: i32,
    /// Item ID to give as reward.
    pub n_give_item_num: i32,
    /// Quantity to give.
    pub n_give_item_count: i16,
    /// Weighted success rate (0–10000, divided by 5 for array slots).
    pub success_rate: i32,
}

/// Sealed item record from the `sealed_items` table.
///
/// Tracks which items a player has sealed/locked.
/// MSSQL source: `SEALED_ITEMS` (249 rows, player data).
/// C++ equivalent: `_SEALED_ITEM`.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SealedItemRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Account ID owning the sealed item.
    pub account_id: String,
    /// Character ID owning the sealed item.
    pub character_id: String,
    /// Item serial number.
    pub item_serial: i64,
    /// Item template ID.
    pub item_id: i32,
    /// Current seal type.
    pub seal_type: i16,
    /// Original seal type before modification.
    pub original_seal_type: i16,
    /// Pre-lock state (0=unlocked).
    pub prelock_state: i16,
}
