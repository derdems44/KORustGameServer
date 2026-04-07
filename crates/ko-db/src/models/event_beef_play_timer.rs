//! BDW event timer configuration.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct EventBeefPlayTimer {
    pub event_local_id: i16,
    pub event_zone_id: i16,
    pub event_name: String,
    pub monument_time: i32,
    pub loser_sign_time: i32,
    pub farming_time: i32,
}
