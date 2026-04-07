//! Knights cape models — maps to `knights_cape`, `knights_cape_castellan_bonus`,
//! `knights_csw_opt`, and `user_knightdata` PostgreSQL tables.
//! - `CKnightsCapeSet` in `KnightsCapeSet.h` — cape definitions
//! - `CCapeCastellanBonusSet` in `CapeCastellanBonusSet.h` — castellan bonuses
//! - `KNIGHTS_CSW_OPT` — castle siege war configuration
//! - `USER_KNIGHTDATA` — per-user clan membership data

/// A single knights cape definition row.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsCapeRow {
    /// Cape index (primary key).
    pub s_cape_index: i16,
    /// Gold cost to purchase.
    pub n_buy_price: i32,
    /// Required clan grade (1-5).
    pub by_grade: i16,
    /// Clan point (loyalty) cost to purchase.
    pub n_buy_loyalty: i32,
    /// Required clan ranking.
    pub by_ranking: i16,
    /// Cape type (0=normal, 1=pattern, 2=emblem, 3=castellan).
    pub b_type: i16,
    /// Whether this cape requires a ticket item (0=no, 1=yes).
    pub b_ticket: i16,
    /// Castellan bonus type index (references knights_cape_castellan_bonus).
    pub bonus_type: i16,
}

/// Castellan cape bonus definition.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsCapeCastellanBonusRow {
    /// Bonus type (primary key, 1 or 2).
    pub bonus_type: i16,
    /// Description label.
    pub type_name: String,
    /// Armor class bonus.
    pub ac_bonus: i16,
    /// Hit point bonus.
    pub hp_bonus: i16,
    /// Mana point bonus.
    pub mp_bonus: i16,
    /// Strength bonus.
    pub str_bonus: i16,
    /// Stamina bonus.
    pub sta_bonus: i16,
    /// Dexterity bonus.
    pub dex_bonus: i16,
    /// Intelligence bonus.
    pub int_bonus: i16,
    /// Charisma bonus.
    pub cha_bonus: i16,
    /// Fire resistance bonus.
    pub flame_resist: i16,
    /// Ice resistance bonus.
    pub glacier_resist: i16,
    /// Lightning resistance bonus.
    pub lightning_resist: i16,
    /// Magic resistance bonus.
    pub magic_resist: i16,
    /// Disease resistance bonus.
    pub disease_resist: i16,
    /// Poison resistance bonus.
    pub poison_resist: i16,
    /// XP bonus percentage.
    pub xp_bonus_pct: i16,
    /// Coin bonus percentage.
    pub coin_bonus_pct: i16,
    /// Attack power bonus percentage.
    pub ap_bonus_pct: i16,
    /// AC bonus percentage.
    pub ac_bonus_pct: i16,
    /// Maximum weight bonus.
    pub max_weight_bonus: i16,
    /// Nation point (NP) bonus.
    pub np_bonus: i16,
}

/// Castle Siege War configuration options (single row).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsCswOptRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Preparation time in minutes.
    pub preparing: i16,
    /// War duration in minutes.
    pub war_time: i16,
    /// Gold cost to register.
    pub money: i32,
    /// TL (Turkish Lira) cost.
    pub tl: i32,
    /// Cash shop cost.
    pub cash: i32,
    /// Loyalty cost to register.
    pub loyalty: i32,
    /// Required item 1 ID.
    pub item_id_1: i32,
    /// Required item 1 count.
    pub item_count_1: i32,
    /// Required item 1 time limit.
    pub item_time_1: i32,
    /// Required item 2 ID.
    pub item_id_2: i32,
    /// Required item 2 count.
    pub item_count_2: i32,
    /// Required item 2 time limit.
    pub item_time_2: i32,
    /// Required item 3 ID.
    pub item_id_3: i32,
    /// Required item 3 count.
    pub item_count_3: i32,
    /// Required item 3 time limit.
    pub item_time_3: i32,
}

/// Per-user clan membership data.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserKnightDataRow {
    /// Clan ID this user belongs to.
    pub s_clan_id: i16,
    /// Character name.
    pub str_user_id: String,
    /// Nation points donated to clan.
    pub n_donated_np: i32,
    /// Personal memo field.
    pub str_memo: String,
    /// Fame/rank within the clan (1=chief, 2=vice chief, 5=member).
    pub fame: i16,
    /// Character class ID.
    pub s_class: i16,
    /// Character level.
    pub level: i16,
    /// Last login unix timestamp.
    pub last_login: i32,
    /// Total loyalty points.
    pub loyalty: i32,
    /// Monthly loyalty points.
    pub loyalty_monthly: i32,
}
