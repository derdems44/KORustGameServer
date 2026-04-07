//! Costume data model — user costume appearance state.

/// Persisted costume state for a single character.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserCostume {
    /// Character ID (primary key).
    pub character_id: String,
    /// Active type: 0=none, 1=available, 2=equipped, 3=expired.
    pub active_type: i16,
    /// Equipped costume item ID.
    pub item_id: i32,
    /// Costume item parameter.
    pub item_param: i32,
    /// Model scale value.
    pub scale_raw: i32,
    /// Dye color index (0-13).
    pub color_index: i16,
    /// Absolute UNIX expiry timestamp (seconds).
    pub expiry_time: i64,
}
