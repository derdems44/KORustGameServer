//! Bot system models — farm bots, merchant bots, ranking data.

/// A row from the `bot_handler_farm` table — defines a farm bot character.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BotHandlerFarmRow {
    /// Unique bot ID (added to MAX_USER for in-game socket ID).
    pub id: i32,
    /// Character name for the bot.
    pub str_user_id: String,
    /// Nation: 1 = Karus, 2 = Elmorad.
    pub nation: i16,
    /// Race code.
    pub race: i16,
    /// Class code (e.g. 107 = rogue mastered).
    pub class: i16,
    /// Hair RGB color value.
    pub hair_rgb: i32,
    /// Character level.
    pub level: i16,
    /// Face type index.
    pub face: i16,
    /// Knights (clan) ID.
    pub knights: i16,
    /// Fame rank.
    pub fame: i16,
    /// Zone ID where bot spawns.
    pub zone: i16,
    /// Position X (multiplied by 100 in MSSQL).
    pub px: i32,
    /// Position Z (multiplied by 100 in MSSQL).
    pub pz: i32,
    /// Position Y (height, multiplied by 100 in MSSQL).
    pub py: i32,
    /// Binary item data (INVENTORY_TOTAL * 8 bytes).
    pub str_item: Option<Vec<u8>>,
    /// Achievement cover title ID.
    pub cover_title: i32,
    /// Rebirth level.
    pub reb_level: i16,
    /// Skill binary data (10 bytes).
    pub str_skill: Option<Vec<u8>>,
    /// Gold amount.
    pub gold: i32,
    /// Stat points remaining.
    pub points: i16,
    /// STR stat.
    pub strong: i16,
    /// STA stat.
    pub sta: i16,
    /// DEX stat.
    pub dex: i16,
    /// INT stat.
    pub intel: i16,
    /// CHA stat.
    pub cha: i16,
    /// National Points (loyalty).
    pub loyalty: i32,
    /// Monthly loyalty points.
    pub loyalty_monthly: i32,
    /// Donated NP to clan.
    pub donated_np: i32,
}

/// A row from the `bot_handler_merchant` table — merchant bot item template.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BotHandlerMerchantRow {
    /// Template index (used as key in m_ArtificialMerchantArray).
    pub s_index: i16,
    /// Merchant type: 0 = selling, 1 = buying (premium merchant).
    pub bot_merchant_type: i16,
    /// Comma-separated item IDs.
    pub bot_item_num: String,
    /// Comma-separated item counts.
    pub bot_item_count: String,
    /// Comma-separated item prices.
    pub bot_item_price: String,
    /// Optional merchant advertisement message.
    pub bot_merchant_message: Option<String>,
}

/// A row from the `bot_merchant_data` table — pre-configured merchant stall.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BotMerchantDataRow {
    /// Unique stall index.
    pub n_index: i32,
    /// Advertisement message shown to players.
    pub advert_message: Option<String>,
    // Slot 1-12: item_id, price, count, duration, is_kc
    pub n_num1: i32,
    pub n_price1: i32,
    pub s_count1: i32,
    pub s_duration1: i32,
    pub is_kc1: bool,
    pub n_num2: i32,
    pub n_price2: i32,
    pub s_count2: i32,
    pub s_duration2: i32,
    pub is_kc2: bool,
    pub n_num3: i32,
    pub n_price3: i32,
    pub s_count3: i32,
    pub s_duration3: i32,
    pub is_kc3: bool,
    pub n_num4: i32,
    pub n_price4: i32,
    pub s_count4: i32,
    pub s_duration4: i32,
    pub is_kc4: bool,
    pub n_num5: i32,
    pub n_price5: i32,
    pub s_count5: i32,
    pub s_duration5: i32,
    pub is_kc5: bool,
    pub n_num6: i32,
    pub n_price6: i32,
    pub s_count6: i32,
    pub s_duration6: i32,
    pub is_kc6: bool,
    pub n_num7: i32,
    pub n_price7: i32,
    pub s_count7: i32,
    pub s_duration7: i32,
    pub is_kc7: bool,
    pub n_num8: i32,
    pub n_price8: i32,
    pub s_count8: i32,
    pub s_duration8: i32,
    pub is_kc8: bool,
    pub n_num9: i32,
    pub n_price9: i32,
    pub s_count9: i32,
    pub s_duration9: i32,
    pub is_kc9: bool,
    pub n_num10: i32,
    pub n_price10: i32,
    pub s_count10: i32,
    pub s_duration10: i32,
    pub is_kc10: bool,
    pub n_num11: i32,
    pub n_price11: i32,
    pub s_count11: i32,
    pub s_duration11: i32,
    pub is_kc11: bool,
    pub n_num12: i32,
    pub n_price12: i32,
    pub s_count12: i32,
    pub s_duration12: i32,
    pub is_kc12: bool,
    /// Position X.
    pub px: i32,
    /// Position Z.
    pub pz: i32,
    /// Position Y.
    pub py: i32,
    /// Duration in minutes (9999 = permanent).
    pub minute: i32,
    /// Zone ID.
    pub zone: i32,
    /// Facing direction.
    pub s_direction: i32,
    /// Merchant type: 0 = selling, 1 = buying.
    pub merchant_type: i16,
}

/// A row from the `user_bots` table — individual user bot definitions.
/// Similar to BotHandlerFarmRow but without loyalty fields.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserBotRow {
    pub id: i32,
    pub str_user_id: String,
    pub nation: i16,
    pub race: i16,
    pub class: i16,
    pub hair_rgb: i32,
    pub level: i16,
    pub face: i16,
    pub knights: i16,
    pub fame: i16,
    pub zone: i16,
    pub px: i32,
    pub pz: i32,
    pub py: i32,
    pub str_item: Option<Vec<u8>>,
    pub cover_title: i32,
    pub reb_level: i16,
    pub str_skill: Option<Vec<u8>>,
    pub gold: i32,
    pub points: i16,
    pub strong: i16,
    pub sta: i16,
    pub dex: i16,
    pub intel: i16,
    pub cha: i16,
}

/// A row from the `bot_knights_rank` table — dual-nation knights ranking.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BotKnightsRankRow {
    pub sh_index: i16,
    pub str_name: String,
    pub str_elmo_user_id: Option<String>,
    pub str_elmo_knights_name: Option<String>,
    pub s_elmo_knights: Option<i16>,
    pub n_elmo_loyalty: Option<i32>,
    pub str_karus_user_id: Option<String>,
    pub str_karus_knights_name: Option<String>,
    pub s_karus_knights: Option<i16>,
    pub n_karus_loyalty: Option<i32>,
    pub n_money: Option<i32>,
}

/// A row from the `bot_personal_rank` table — dual-nation personal ranking.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BotPersonalRankRow {
    pub n_rank: i16,
    pub str_rank_name: String,
    pub n_elmo_up: i16,
    pub str_elmo_user_id: Option<String>,
    pub str_elmo_clan_name: Option<String>,
    pub s_elmo_knights: Option<i16>,
    pub n_elmo_loyalty_monthly: Option<i32>,
    pub n_elmo_check: i32,
    pub n_karus_up: i16,
    pub str_karus_user_id: Option<String>,
    pub str_karus_clan_name: Option<String>,
    pub s_karus_knights: Option<i16>,
    pub n_karus_loyalty_monthly: Option<i32>,
    pub n_karus_check: i32,
    pub n_salary: i32,
    pub update_date: chrono::NaiveDateTime,
}
