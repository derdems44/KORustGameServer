//! Knights (clan) model (maps to `knights` table).

use chrono::{DateTime, Utc};

/// A clan / knights entry.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Knights {
    pub id_num: i16,
    pub flag: i16,
    pub nation: i16,
    pub ranking: i16,
    pub id_name: String,
    pub members: i32,
    pub chief: String,
    pub vice_chief_1: Option<String>,
    pub vice_chief_2: Option<String>,
    pub vice_chief_3: Option<String>,
    pub gold: i64,
    pub domination: i16,
    pub points: i32,
    pub mark: Vec<u8>,
    pub s_mark_version: i16,
    pub s_mark_len: i16,
    pub s_cape: i16,
    pub b_cape_r: i16,
    pub b_cape_g: i16,
    pub b_cape_b: i16,
    pub s_cast_cape: i16,
    pub b_cast_cape_r: i16,
    pub b_cast_cape_g: i16,
    pub b_cast_cape_b: i16,
    pub b_cast_time: i32,
    pub s_alliance_knights: i16,
    pub clan_point_fund: i32,
    pub str_clan_notice: Option<String>,
    pub by_siege_flag: i16,
    pub n_lose: i16,
    pub n_victory: i16,
    pub clan_point_method: i16,
    pub n_money: i32,
    pub dw_time: i32,
    pub warehouse_data: Vec<u8>,
    pub str_serial: Vec<u8>,
    pub s_premium_time: i32,
    pub s_premium_in_use: i16,
    pub dt_create_time: Option<DateTime<Utc>>,
}

/// A knights alliance entry (maps to `knights_alliance` table).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct KnightsAllianceRow {
    pub s_main_alliance_knights: i16,
    pub s_sub_alliance_knights: i16,
    pub s_mercenary_clan_1: i16,
    pub s_mercenary_clan_2: i16,
    pub str_alliance_notice: String,
}
