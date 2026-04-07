//! Guild Bank data models — clan shared storage state.

/// Guild bank settings for a clan.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GuildBankRow {
    /// Clan ID (primary key).
    pub knights_id: i32,
    /// Stored gold.
    pub gold: i64,
    /// Number of unlocked tabs (1-9).
    pub max_tabs: i16,
    /// Default member permission flags.
    pub permissions: i16,
}

/// A single item slot in the guild bank.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GuildBankItemRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Clan ID.
    pub knights_id: i32,
    /// Tab index (0-8).
    pub tab_index: i16,
    /// Slot within tab.
    pub slot_id: i32,
    /// Item template ID.
    pub item_id: i32,
    /// Stack count.
    pub item_count: i16,
    /// Maximum durability.
    pub max_durability: i16,
    /// Current durability.
    pub cur_durability: i16,
    /// Item flags.
    pub flag: i16,
    /// Item expiry (seconds).
    pub expiry_time: i32,
}

/// A transaction log entry.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GuildBankLogRow {
    /// Auto-increment ID.
    pub id: i32,
    /// Clan ID.
    pub knights_id: i32,
    /// Character who performed the action.
    pub character_id: String,
    /// Tab index.
    pub tab_index: i16,
    /// Item template ID.
    pub item_id: i32,
    /// Quantity.
    pub quantity: i16,
    /// Price.
    pub price: i32,
    /// Action type (1=deposit, 2=withdraw).
    pub action_type: i16,
}
