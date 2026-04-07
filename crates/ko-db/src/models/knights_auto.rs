//! Auto-created clan templates.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct KnightsAuto {
    pub nation: i16,
    pub status: i16,
    pub clan_id: i32,
    pub clan_name: String,
    pub flag: i16,
    pub account_id: String,
    pub password: String,
    pub chief: String,
    pub mark: Option<Vec<u8>>,
    pub mark_len: i32,
    pub mark_ver: i16,
    pub cape_id: i16,
    pub cape_r: i16,
    pub cape_g: i16,
    pub cape_b: i16,
    pub clan_notice: Option<String>,
    pub test: i16,
}
