//! Cinderella War (Fun Class) event model structs.
//! Maps to PostgreSQL tables: `cindwar_setting`, `cindwar_items`,
//! `cindwar_reward`, `cindwar_reward_item`, `cindwar_stat`.

use sqlx::FromRow;

/// A tier configuration for the Cinderella War event.
#[derive(Debug, Clone, FromRow)]
pub struct CindwarSettingRow {
    pub setting_id: i16,
    pub playtime: i32,
    pub preparetime: i32,
    pub min_level: i16,
    pub max_level: i16,
    pub req_money: i32,
    pub req_loyalty: i32,
    pub max_user_limit: i16,
    pub zone_id: i16,
    pub beginner_level: i16,
}

/// An equipment entry for a Cinderella War tier+class combo.
#[derive(Debug, Clone, FromRow)]
pub struct CindwarItemRow {
    pub tier: i16,
    pub id: i32,
    pub class: i16,
    pub slot_id: i16,
    pub item_id: i32,
    pub item_count: i16,
    pub item_duration: i16,
    pub item_flag: i16,
    pub item_expire: i32,
}

/// Rank-based reward for Cinderella War event.
#[derive(Debug, Clone, FromRow)]
pub struct CindwarRewardRow {
    pub rank_id: i16,
    pub exp_count: i32,
    pub cash_count: i32,
    pub loyalty_count: i32,
    pub money_count: i32,
}

/// A reward item associated with a rank.
/// Normalized from the C++ flat `itemid[10]`/`itemcount[10]` arrays.
#[derive(Debug, Clone, FromRow)]
pub struct CindwarRewardItemRow {
    pub rank_id: i16,
    pub slot: i16,
    pub item_id: i32,
    pub item_count: i32,
    pub item_duration: i32,
    pub item_expiration: i32,
}

/// Per-class stat/skill preset for a Cinderella War tier.
#[derive(Debug, Clone, FromRow)]
pub struct CindwarStatRow {
    pub id: i32,
    pub setting_id: i16,
    pub class: i16,
    pub skill_freepoint: i16,
    pub skill_page1: i16,
    pub skill_page2: i16,
    pub skill_page3: i16,
    pub skill_page4: i16,
    pub stat_str: i16,
    pub stat_sta: i16,
    pub stat_dex: i16,
    pub stat_int: i16,
    pub stat_cha: i16,
    pub stat_freepoint: i16,
}
