//! Wheel of Fun event items and settings.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct WheelOfFunItem {
    pub id: i16,
    pub name: String,
    pub num: i32,
    pub count: i32,
    pub percent: i32,
    pub days: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct WheelOfFunSettings {
    pub idx: i16,
    pub item_name: String,
    pub item_id: i32,
    pub item_count: i32,
    pub rental_time: i32,
    pub flag: i16,
    pub drop_rate: i32,
}
