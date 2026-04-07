//! Character-related models: USERDATA, USER_ITEMS.

use chrono::{DateTime, Utc};

/// A player character (maps to `userdata` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserData {
    pub id: i32,
    pub str_user_id: String,
    pub nation: i16,
    pub race: i16,
    pub class: i16,
    pub hair_rgb: i32,
    pub rank: i16,
    pub title: i16,
    pub level: i16,
    pub rebirth_level: i16,
    pub exp: i64,
    pub loyalty: i32,
    pub face: i16,
    pub city: i16,
    pub knights: i16,
    pub fame: i16,
    pub hp: i16,
    pub mp: i16,
    pub sp: i16,
    pub strong: i16,
    pub sta: i16,
    pub dex: i16,
    pub intel: i16,
    pub cha: i16,
    pub reb_str: i16,
    pub reb_sta: i16,
    pub reb_dex: i16,
    pub reb_intel: i16,
    pub reb_cha: i16,
    pub authority: i16,
    pub points: i16,
    pub gold: i32,
    pub zone: i16,
    pub bind: i16,
    pub bind_px: i32,
    pub bind_pz: i32,
    pub px: i32,
    pub pz: i32,
    pub py: i32,
    pub dw_time: i32,
    pub str_skill: Option<String>,
    pub skill0: i16,
    pub skill1: i16,
    pub skill2: i16,
    pub skill3: i16,
    pub skill4: i16,
    pub skill5: i16,
    pub skill6: i16,
    pub skill7: i16,
    pub skill8: i16,
    pub skill9: i16,
    pub manner_point: i32,
    pub loyalty_monthly: i32,
    pub i_saved_cont: i32,
    pub dt_create_time: DateTime<Utc>,
    pub dt_update_time: Option<DateTime<Utc>>,
    pub n_last_login: i32,
    pub n_donated_np: i32,
    pub mute_status: i32,
    pub attack_status: i32,
    pub tagname: String,
    pub tagname_rgb: i32,
    pub chicken_status: i16,
    pub flash_time: i32,
    pub flash_count: i16,
    pub flash_type: i16,
    pub str_memo: Option<String>,
}

/// A single inventory slot (maps to `user_items` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserItem {
    pub id: i64,
    pub str_user_id: String,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}

/// A deleted inventory item (maps to `user_deleted_items` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDeletedItem {
    pub id: i64,
    pub str_user_id: String,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub deleted_at: DateTime<Utc>,
}

/// A trashed (sold) item available for repurchase (maps to `trash_item_list` table).
/// Non-countable items sold to NPC merchants are stored here for 72 minutes
/// so the player can buy them back at cost * 30.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TrashItemRow {
    pub id: i64,
    pub str_user_id: String,
    pub item_id: i32,
    pub delete_time: i32,
    pub duration: i16,
    pub count: i32,
    pub flag: i16,
    pub serial_num: i64,
}

/// A single warehouse slot (maps to `user_warehouse` table).
/// Warehouse is per-account (not per-character).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WarehouseItem {
    pub id: i64,
    pub str_account_id: String,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}

/// Warehouse coins (gold stored in the inn).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WarehouseCoins {
    pub str_account_id: String,
    pub coins: i32,
}

/// VIP warehouse metadata (maps to `vip_warehouse` table).
/// `m_bWIPStorePassowrdRequest` in `User.h`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VipWarehouseRow {
    pub str_account_id: String,
    pub password: String,
    pub password_request: i16,
    pub vault_expiry: i32,
}

/// A single VIP warehouse item slot (maps to `vip_warehouse_items` table).
/// VIP Warehouse is per-account (not per-character), with 48 slots.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VipWarehouseItemRow {
    pub id: i64,
    pub str_account_id: String,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}
