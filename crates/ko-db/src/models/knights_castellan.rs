//! Castellan clan cape bonus data.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct KnightsCastellan {
    pub id_num: i16,
    pub cape: i16,
    pub cape_r: i16,
    pub cape_g: i16,
    pub cape_b: i16,
    pub is_active: bool,
    pub remaining_time: i64,
}
