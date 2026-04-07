//! Game master settings model — maps to the `game_master_settings` PostgreSQL table.
//!
//! Source: MSSQL `GAME_MASTER_SETTINGS` table — per-GM granular permission flags.

/// A row from the `game_master_settings` table — defines per-character
/// GM permission flags.
///
/// Each boolean field controls whether the GM can perform a specific
/// administrative action. A value of `true` grants the permission.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct GameMasterSettings {
    /// Character ID of the game master (PK).
    pub char_id: String,

    // --- Moderation permissions ---
    /// Can mute a player.
    pub perm_mute: bool,
    /// Can unmute a player.
    pub perm_unmute: bool,
    /// Can unban a player.
    pub perm_unban: bool,
    /// Can issue a ban permit.
    pub perm_ban_permit: bool,
    /// Can ban players under a certain level.
    pub perm_ban_under: bool,
    /// Can issue time-limited bans (days).
    pub perm_ban_days: bool,
    /// Can ban for cheating.
    pub perm_ban_cheating: bool,
    /// Can ban for scamming.
    pub perm_ban_scamming: bool,

    // --- Combat permissions ---
    /// Can enable attack mode for a target.
    pub perm_allow_attack: bool,
    /// Can disable attack mode for a target.
    pub perm_disabled_attack: bool,

    // --- Rate/currency permissions ---
    /// Can add nation points (NP).
    pub perm_np_add: bool,
    /// Can add experience points.
    pub perm_exp_add: bool,
    /// Can add gold/money.
    pub perm_money_add: bool,
    /// Can modify drop rates.
    pub perm_drop_add: bool,
    /// Can change loyalty points.
    pub perm_loyalty_change: bool,
    /// Can change experience values.
    pub perm_exp_change: bool,
    /// Can change money values.
    pub perm_money_change: bool,

    // --- Item permissions ---
    /// Can give items to other players.
    pub perm_give_item: bool,
    /// Can give items to self.
    pub perm_give_item_self: bool,

    // --- Teleport/movement permissions ---
    /// Can summon a user to GM's location.
    pub perm_summon_user: bool,
    /// Can teleport to a user's location.
    pub perm_tp_on_user: bool,
    /// Can change zones.
    pub perm_zone_change: bool,
    /// Can change location within a zone.
    pub perm_location_change: bool,

    // --- Spawn permissions ---
    /// Can summon monsters.
    pub perm_monster_summon: bool,
    /// Can summon NPCs.
    pub perm_npc_summon: bool,
    /// Can kill all monsters.
    pub perm_mon_killed: bool,

    // --- Mass action permissions ---
    /// Can teleport all users.
    pub perm_teleport_all_user: bool,
    /// Can summon an entire clan.
    pub perm_clan_summon: bool,

    // --- System permissions ---
    /// Can reset rankings.
    pub perm_reset_ranking: bool,
    /// Can change event room settings.
    pub perm_change_event_room: bool,
    /// Can open a war event.
    pub perm_war_open: bool,
    /// Can close a war event.
    pub perm_war_close: bool,
    /// Can trigger captain election.
    pub perm_captain_election: bool,
    /// Can send players to prison.
    pub perm_send_prison: bool,
    /// Can change KC (Knight Cash) values.
    pub perm_kc_change: bool,
    /// Can reload server tables at runtime.
    pub perm_reload_tables: bool,
    /// Can run drop tests.
    pub perm_drop_test: bool,
}
