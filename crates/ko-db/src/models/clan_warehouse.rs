/// Clan warehouse item row (maps to `clan_warehouse_items` table).
///
/// C++ Reference: `CKnights::m_sClanWarehouseArray[WAREHOUSE_MAX]`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ClanWarehouseItemRow {
    pub id: i64,
    pub clan_id: i16,
    pub slot_index: i16,
    pub item_id: i32,
    pub durability: i16,
    pub count: i16,
    pub flag: i16,
    pub original_flag: i16,
    pub serial_num: i64,
    pub expire_time: i32,
}
