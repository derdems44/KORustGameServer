//! Monster resource strings and death notices.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct MonsterResource {
    pub sid: i16,
    pub sid_name: String,
    pub resource: String,
    pub notice_zone: i16,
    pub notice_type: i16,
}
