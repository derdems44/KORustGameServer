//! Character Seal system models.
//!
//! C++ Reference: `SealHandler.cpp` — `_CHARACTER_SEAL_ITEM` / `_CHARACTER_SEAL_ITEM_MAPPING`

/// A sealed character snapshot row (maps to `character_seal_items` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CharacterSealItemRow {
    pub id: i32,
    pub account_id: String,
    pub char_name: String,
    pub race: i16,
    pub class: i16,
    pub level: i16,
    pub rebirth_level: i16,
    pub face: i16,
    pub hair_rgb: i32,
    pub rank: i16,
    pub title: i16,
    pub exp: i64,
    pub loyalty: i32,
    pub loyalty_monthly: i32,
    pub manner_point: i32,
    pub fame: i16,
    pub city: i16,
    pub knights: i16,
    pub hp: i16,
    pub mp: i16,
    pub sp: i16,
    pub zone_id: i16,
    pub strong: i16,
    pub sta: i16,
    pub dex: i16,
    pub intel: i16,
    pub cha: i16,
    pub authority: i16,
    pub free_points: i16,
    pub gold: i32,
    pub skill_cat1: i16,
    pub skill_cat2: i16,
    pub skill_cat3: i16,
    pub skill_master: i16,
    pub inventory_data: Option<Vec<u8>>,
    pub item_serial: i64,
}

/// A mapping from cypher ring unique_id to sealed character item
/// (maps to `character_seal_mapping` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CharacterSealMappingRow {
    pub id: i32,
    pub unique_id: i32,
    pub seal_item_id: i32,
    pub account_id: String,
}
