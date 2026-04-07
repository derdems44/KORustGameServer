//! Premium system models — maps to `premium_item_types`, `premium_item_exp`,
//! `account_premium`, and `premium_gift_item` PostgreSQL tables.
//! - `_PREMIUM_ITEM` struct — per-type bonus definitions
//! - `_PREMIUM_ITEM_EXP` struct — level-range XP bonus per premium type
//! - `ACCOUNT_PREMIUM_DATA` — per-account active subscriptions
//! Premium types (1-13) define what bonuses a player receives while that
//! premium tier is active: XP restore on death, gold bonus, drop bonus,
//! loyalty bonus, repair discount, and sell price bonus.

/// A premium item type definition row from the database.
/// Defines the bonus percentages for each premium tier.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PremiumItemRow {
    /// Premium type identifier (1-13).
    pub premium_type: i16,
    /// Display name.
    pub name: String,
    /// XP restore percent on death (float, e.g., 2.0 = 2% of max_exp lost instead of 5%).
    ///
    /// C++ field: `ExpRestorePercent`
    pub exp_restore_pct: f64,
    /// Gold (Noah) gain bonus percent from monster drops.
    ///
    /// C++ field: `NoahPercent`
    pub noah_pct: i16,
    /// Item drop rate bonus percent.
    ///
    /// C++ field: `DropPercent`
    pub drop_pct: i16,
    /// Flat bonus loyalty (NP) per PK kill.
    ///
    /// C++ field: `BonusLoyalty`
    pub bonus_loyalty: i32,
    /// Repair cost discount (e.g., 50 = pay 50% of normal cost).
    ///
    /// C++ field: `RepairDiscountPercent`
    pub repair_disc_pct: i16,
    /// If > 0, sell price uses buy_price/4 instead of buy_price/6.
    ///
    /// C++ field: `ItemSellPercent`
    pub item_sell_pct: i16,
}

/// Premium XP bonus by level range for a specific premium type.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PremiumItemExpRow {
    /// Unique index.
    pub n_index: i16,
    /// Premium type this entry applies to.
    pub premium_type: i16,
    /// Minimum player level (inclusive).
    pub min_level: i16,
    /// Maximum player level (inclusive).
    pub max_level: i16,
    /// XP bonus percentage (e.g., 100 = +100% XP).
    pub s_percent: i16,
}

/// A premium gift item row from the database.
/// Bonus items automatically sent to players via letter when a premium
/// of the matching type is activated.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PremiumGiftItemRow {
    /// Row identifier.
    pub id: i32,
    /// Premium type this gift belongs to.
    pub premium_type: Option<i16>,
    /// Item ID to give.
    pub bonus_item_num: Option<i32>,
    /// Number of items to give.
    pub count: Option<i16>,
    /// Sender name in the letter.
    pub sender: Option<String>,
    /// Subject line of the letter.
    pub subject: Option<String>,
    /// Message body of the letter.
    pub message: Option<String>,
    /// Item display name (informational).
    pub item_name: Option<String>,
}

/// Per-account premium subscription slot (normalized from MSSQL blob).
/// Each account can have up to 6 premium slots. The `premium_type` and
/// `expiry_time` (Unix timestamp) define what premium is available.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountPremiumRow {
    /// Account ID (varchar).
    pub account_id: String,
    /// Slot index (0-5).
    pub slot: i16,
    /// Premium type stored in this slot (0 = empty).
    pub premium_type: i16,
    /// Unix timestamp when this premium expires (0 = inactive).
    pub expiry_time: i32,
}
