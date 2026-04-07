//! User data persistence models — genie, daily ops, loot settings, seal exp, return data.
//!
//! C++ Reference: Various `USER_*` tables in MSSQL (per-user/per-character data).

/// A row from the `user_genie_data` table — genie persistence per user.
///
/// C++ Reference: `CUser::m_GenieOptions`, `m_1098GenieTime`, `m_sFirstUsingGenie`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserGenieDataRow {
    /// User account ID (PK).
    pub user_id: String,
    /// Remaining genie time in seconds (Unix timestamp-based).
    pub genie_time: i32,
    /// Binary genie options blob (100 bytes in MSSQL).
    pub genie_options: Vec<u8>,
    /// Whether the user has used the genie before (0=no, 1=yes).
    pub first_using_genie: i16,
}

/// A row from the `user_daily_op` table — daily activity cooldowns per user.
///
/// C++ Reference: Various `m_*Time` fields on CUser, persisted via USER_DAILY_OP.
/// All time values are Unix timestamps; -1 means "not yet used".
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserDailyOpRow {
    /// User account ID (PK).
    pub user_id: String,
    /// Chaos map entry timestamp.
    pub chaos_map_time: i32,
    /// User rank reward claim timestamp.
    pub user_rank_reward_time: i32,
    /// Personal rank reward claim timestamp.
    pub personal_rank_reward_time: i32,
    /// King wing reward timestamp.
    pub king_wing_time: i32,
    /// Warder/killer time slot 1.
    pub warder_killer_time1: i32,
    /// Warder/killer time slot 2.
    pub warder_killer_time2: i32,
    /// Keeper killer time.
    pub keeper_killer_time: i32,
    /// User loyalty wing reward time.
    pub user_loyalty_wing_reward_time: i32,
    /// Full Moon Rift map entry timestamp.
    pub full_moon_rift_map_time: i32,
    /// Copy information time.
    pub copy_information_time: i32,
}

/// A row from the `user_loot_settings` table — auto-loot filter preferences.
///
/// C++ Reference: USER_LOOT_SETTINGS table, filters by class, item type, and price.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserLootSettingsRow {
    /// Auto-increment primary key.
    pub id: i32,
    /// User account ID (unique).
    pub user_id: String,
    /// Filter: pick up warrior items (1=yes, 0=no).
    pub warrior: i16,
    /// Filter: pick up rogue items.
    pub rogue: i16,
    /// Filter: pick up mage items.
    pub mage: i16,
    /// Filter: pick up priest items.
    pub priest: i16,
    /// Filter: pick up weapons.
    pub weapon: i16,
    /// Filter: pick up armor.
    pub armor: i16,
    /// Filter: pick up accessories.
    pub accessory: i16,
    /// Filter: pick up normal-grade items.
    pub normal: i16,
    /// Filter: pick up upgrade materials.
    pub upgrade: i16,
    /// Filter: pick up crafting materials.
    pub craft: i16,
    /// Filter: pick up rare items.
    pub rare: i16,
    /// Filter: pick up magic items.
    pub magic: i16,
    /// Filter: pick up unique items.
    pub unique_grade: i16,
    /// Filter: pick up consumables.
    pub consumable: i16,
    /// Minimum price threshold for auto-loot.
    pub price: i32,
}

/// A row from the `user_seal_exp` table — sealed (banked) experience.
///
/// C++ Reference: USER_SEAL_EXP table, `sSealedExp` field.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserSealExpRow {
    /// User account ID (PK).
    pub user_id: String,
    /// Sealed experience amount.
    pub sealed_exp: i32,
}

/// A row from the `user_return_data` table — returning player data.
///
/// C++ Reference: USER_RETURN_DATA table, tracks return symbol eligibility.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserReturnDataRow {
    /// Character ID (PK).
    pub character_id: String,
    /// Whether the return symbol is active (0=no, 1=yes).
    pub return_symbol_ok: Option<i16>,
    /// Logout time as Unix timestamp.
    pub return_logout_time: Option<i64>,
    /// Return symbol activation time as Unix timestamp.
    pub return_symbol_time: Option<i64>,
}

// DailyRewardUserRow is defined in models::daily_reward (4 fields including day_of_month).
// Re-exported here for backwards compatibility.
pub use super::daily_reward::DailyRewardUserRow;
