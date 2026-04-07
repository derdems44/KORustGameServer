//! Mining & Fishing item drop table model.
//!
//! C++ Reference: `_MINING_FISHING_ITEM` struct, loaded from `MINING_FISHING_ITEM` table.

/// A row from the `mining_fishing_item` table — defines possible rewards
/// for mining and fishing activities.
///
/// C++ Reference: `_MINING_FISHING_ITEM` in `MiningFishingTableSet.h`
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MiningFishingItemRow {
    /// Primary key index.
    pub n_index: i32,
    /// Table type: 0 = mining, 1 = fishing.
    pub n_table_type: i32,
    /// War status: 0 = normal, 1 = war loser, 2 = war winner.
    pub n_war_status: i32,
    /// Tool type: 0 = normal tool, 1 = golden tool, 4 = moradon(?).
    pub use_item_type: i16,
    /// Display name of the reward item.
    pub n_give_item_name: String,
    /// Item ID of the reward.
    pub n_give_item_id: i32,
    /// Number of items to give.
    pub n_give_item_count: i32,
    /// Weighted success rate (out of ~10000 pool).
    pub success_rate: i32,
}
