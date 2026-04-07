//! Beginner/new player protection settings.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct BeginnerSettings {
    pub server_no: i16,
    pub beginner_type: i16,
    pub description: Option<String>,
}
