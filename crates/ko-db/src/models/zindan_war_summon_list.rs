//! Zindan War monster summon list.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ZindanWarSummon {
    pub idx: i32,
    pub summon_type: i16,
    pub stage: i16,
    pub sid: i16,
    pub sid_count: i16,
    pub pos_x: i16,
    pub pos_z: i16,
    pub range: i16,
    pub summon_name: Option<String>,
}
