//! Level merchant EXP reward configuration.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LevelMerchantRewards {
    pub idx: i16,
    pub start_hour: i16,
    pub start_minute: i16,
    pub finish_time: i16,
    pub rate_experience: i32,
    pub exp_minute: i32,
}
