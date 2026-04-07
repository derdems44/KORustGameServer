//! Lottery event settings.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LotteryEventSettings {
    pub lnum: i16,
    pub req_item1: i32,
    pub req_item_count1: i32,
    pub req_item2: i32,
    pub req_item_count2: i32,
    pub req_item3: i32,
    pub req_item_count3: i32,
    pub req_item4: i32,
    pub req_item_count4: i32,
    pub req_item5: i32,
    pub req_item_count5: i32,
    pub reward_item1: i32,
    pub reward_item2: i32,
    pub reward_item3: i32,
    pub reward_item4: i32,
    pub user_limit: i32,
    pub event_time: i32,
}
