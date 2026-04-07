//! Zone reward models — maps to `zone_kill_reward` and `zone_online_reward` PostgreSQL tables.
//!
//! Source: MSSQL `ZONE_KILL_REWARD`, `ZONE_ONLINE_REWARD` tables — zone-based player rewards.

/// A row from the `zone_kill_reward` table — defines item rewards
/// granted for kills in specific zones.
///
/// Each row specifies a zone, nation filter, kill count threshold,
/// reward item, and distribution rules (party, priest, warehouse).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ZoneKillReward {
    /// Primary key index.
    pub idx: i32,
    /// Zone ID where kills are counted.
    pub zone_id: i16,
    /// Nation filter (0=both, 1=Karus, 2=Elmorad).
    pub nation: i16,
    /// Whether a party is required (0=no, 1=yes).
    pub party_required: i16,
    /// Whether the reward is given to all party members.
    pub all_party_reward: bool,
    /// Number of kills required to earn the reward.
    pub kill_count: i16,
    /// Display name of the reward item (optional, for admin reference).
    pub item_name: Option<String>,
    /// Item template ID of the reward.
    pub item_id: i32,
    /// Duration of the rewarded item (0=permanent).
    pub item_duration: i16,
    /// Number of items to give.
    pub item_count: i32,
    /// Item flags (bound, tradeable, etc.).
    pub item_flag: i16,
    /// Item expiration time in minutes (0=no expiry).
    pub item_expiration: i32,
    /// Drop rate weight for this reward (percentage or weighted).
    pub drop_rate: i16,
    /// Whether the item is placed directly into the warehouse.
    pub give_to_warehouse: bool,
    /// Reward status (0=disabled, 1=enabled).
    pub status: i16,
    /// Whether this reward is priest-specific.
    pub is_priest: bool,
    /// Priest-specific drop rate modifier.
    pub priest_rate: i16,
}

/// A row from the `zone_online_reward` table — defines rewards granted
/// for being online in a specific zone.
///
/// Each row specifies periodic rewards (items, loyalty, cash) for players
/// who remain online in the zone. "pre_" prefixed fields define premium
/// player rewards.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ZoneOnlineReward {
    /// Zone ID where the online reward applies.
    pub zone_id: i16,
    /// Item template ID of the reward.
    pub item_id: i32,
    /// Number of items to give.
    pub item_count: i32,
    /// Item time/duration parameter.
    pub item_time: i32,
    /// Interval in minutes between reward grants.
    pub minute: i32,
    /// Loyalty points to grant.
    pub loyalty: i32,
    /// Cash (KC) to grant.
    pub cash: i32,
    /// TL (Turkish Lira / real-money equivalent) to grant.
    pub tl: i32,
    /// Premium player: item template ID.
    pub pre_item_id: i32,
    /// Premium player: item count.
    pub pre_item_count: i32,
    /// Premium player: item time/duration.
    pub pre_item_time: i32,
    /// Premium player: interval in minutes.
    pub pre_minute: i32,
    /// Premium player: loyalty points.
    pub pre_loyalty: i32,
    /// Premium player: cash (KC).
    pub pre_cash: i32,
    /// Premium player: TL amount.
    pub pre_tl: i32,
}
