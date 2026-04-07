//! Pet talk message templates.
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PetTalk {
    pub idx: i16,
    pub word: String,
    pub message: String,
    pub emo: String,
    pub rand: i32,
}
