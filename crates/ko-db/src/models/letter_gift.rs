//! Gift letter templates.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LetterGift {
    pub id: i32,
    pub class: i16,
    pub gift_type: i16,
    pub sender_id: String,
    pub item_name: String,
    pub item_description: String,
    pub letter_type: i16,
    pub item_id: i32,
    pub item_count: i32,
    pub item_duration: i32,
    pub sending_status: i16,
    pub expire_time: i32,
    pub serial_num: String,
}
