//! Collection Race event settings and rewards.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CollectionRaceSettings {
    pub event_index: i16,
    pub event_name: String,
    pub unit1: i16,
    pub unit_count1: i16,
    pub unit2: i16,
    pub unit_count2: i16,
    pub unit3: i16,
    pub unit_count3: i16,
    pub min_level: i16,
    pub max_level: i16,
    pub event_zone: i16,
    pub event_time: i32,
    pub user_limit: i32,
    pub is_repeat: bool,
    pub auto_start: bool,
    pub auto_hour: i16,
    pub auto_minute: i16,
}

#[derive(Debug, Clone, FromRow)]
pub struct CollectionRaceReward {
    pub idx: i16,
    pub event_id: i16,
    pub description: String,
    pub item_id: i32,
    pub item_count: i32,
    pub rate: i32,
    pub item_time: i32,
    pub item_flag: i32,
    pub item_session: i32,
}
