//! Clan-vs-Clan (CvC) Tournament system handler.
//! ## Overview
//! The tournament system hosts two types of Clan vs. Clan events:
//! - **Clan War** (zones 77 = Ardream CvC, 78 = Ronark CvC)
//! - **Party vs. Party** (zones 96-99)
//! A GM admin starts a tournament by specifying two clan names and a zone ID.
//! The server tracks scores on a per-zone `TournamentState` record and
//! broadcasts score/timer updates to all players in the arena via
//! `WIZ_BATTLE_EVENT (0x57)` and `WIZ_BIFROST (0x7B)`.
//! ## Packet Formats (all little-endian)
//! ### Score broadcast — WIZ_BATTLE_EVENT (0x57)
//! ```text
//! u8  sub_opcode = 0x12
//! u8  board_type = 2   (Board1) or 3 (Board2)
//! u16 red_score
//! u16 blue_score
//! u32 timer_secs
//! u8  monument_killed_advantage
//! ```
//! ### Timer broadcast — WIZ_BIFROST (0x7B)
//! ```text
//! u8  sub_type = 5
//! u16 timer_secs
//! ```
//! ### KnightsVsList info — WIZ_KNIGHTS_PROCESS (0x6E), sub-opcode 96
//! Sent on zone login to inform the client about the two competing clans.
//! ```text
//! SByte  (packet size prefix, handled by Packet writer)
//! u8     zone_id
//! u16    red_clan_id
//! u16    red_mark_version (sent twice per C++ comment)
//! u16    red_mark_version (duplicate)
//! str    red_clan_name
//! str    red_clan_name   (duplicate)
//! u16    blue_clan_id
//! u16    blue_mark_version
//! u16    blue_mark_version
//! str    blue_clan_name
//! str    blue_clan_name
//! ```
//! ## Tournament Zones
//! | Zone ID | Type              |
//! |---------|-------------------|
//! |  77     | Clan War — Ardream  |
//! |  78     | Clan War — Ronark   |
//! |  96     | Party VS 1          |
//! |  97     | Party VS 2          |
//! |  98     | Party VS 3          |
//! |  99     | Party VS 4          |
//! ## Timer / State Machine
//! - `is_started=true, timer > 0` → battle in progress, scores track kills
//! - `is_started=false` → battle ended, `out_timer` counts down 300s grace period
//! - `is_finished=true && out_timer elapsed` → KickOutZone → delete entry
//! ## Monument Mechanic
//! When a monument is killed by the LOSING clan, they earn half the score gap
//! as bonus points. See `TournamentMonumentKillProcess` in C++ source.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use dashmap::DashMap;
use tracing::debug;

use ko_protocol::{Opcode, Packet};

use super::monument::CHAT_WAR_SYSTEM as WAR_SYSTEM_CHAT;

use crate::world::{
    KnightsInfo, WorldState, ZONE_CLAN_WAR_ARDREAM, ZONE_CLAN_WAR_RONARK, ZONE_MORADON,
    ZONE_PARTY_VS_1, ZONE_PARTY_VS_2, ZONE_PARTY_VS_3, ZONE_PARTY_VS_4,
};
use crate::zone::SessionId;

// ── Constants ─────────────────────────────────────────────────────────────

pub const TOURNAMENT_MIN_CLAN_COUNT: u16 = 1;

pub const TOURNAMENT_MAX_CLAN_COUNT: u16 = 50;

pub const TOURNAMENT_MIN_PARTY_COUNT: u8 = 8;

pub const TOURNAMENT_MAX_PARTY_COUNT: u8 = 8;

/// Grace period (seconds) after a tournament ends before players are kicked.
pub const TOURNAMENT_OUT_TIMER_SECS: u64 = 300;

/// WIZ_KNIGHTS_PROCESS sub-opcode for tournament clan list.
pub const KNIGHTS_VS_LIST_OPCODE: u8 = 96;

/// WIZ_BATTLE_EVENT sub-opcode for tournament score update.
pub const BATTLE_EVENT_TOURNAMENT_SCORE: u8 = 0x12;

/// WIZ_BIFROST sub-type for tournament timer.
pub const BIFROST_TOURNAMENT_TIMER: u8 = 5;

/// Tournament arena zones (clan war + party vs).
pub const TOURNAMENT_ZONES: [u16; 6] = [
    ZONE_CLAN_WAR_ARDREAM,
    ZONE_CLAN_WAR_RONARK,
    ZONE_PARTY_VS_1,
    ZONE_PARTY_VS_2,
    ZONE_PARTY_VS_3,
    ZONE_PARTY_VS_4,
];

// ── Types ──────────────────────────────────────────────────────────────────

/// Tournament arena zone state.
/// Mirrors the `_TOURNAMENT_DATA` struct defined in `GameDefine.h:3759-3789`.
/// ```cpp
/// struct _TOURNAMENT_DATA {
///     uint8  aTournamentZoneID;
///     uint16 aTournamentClanNum[2];
///     uint16 aTournamentScoreBoard[2];
///     uint32 aTournamentTimer;
///     uint8  aTournamentMonumentKilled;
///     time_t aTournamentOutTimer;
///     bool   aTournamentisAttackable;
///     bool   aTournamentisStarted;
///     bool   aTournamentisFinished;
/// };
/// ```
#[derive(Debug, Clone)]
pub struct TournamentState {
    /// Arena zone ID (77/78/96-99).
    pub zone_id: u16,
    /// Clan IDs: [0]=Red, [1]=Blue.
    pub clan_num: [u16; 2],
    /// Scores: [0]=Red, [1]=Blue.
    pub score_board: [u16; 2],
    /// Remaining battle duration in seconds.
    pub timer_secs: u32,
    /// Monument-kill advantage flag (non-zero = monument was killed).
    pub monument_killed: u8,
    /// Unix timestamp after which finished players are kicked.
    pub out_timer: u64,
    /// Whether attacks are allowed (battle phase started).
    pub is_attackable: bool,
    /// Whether the battle is currently running.
    pub is_started: bool,
    /// Whether the battle has finished (waiting for out_timer).
    pub is_finished: bool,
}

impl TournamentState {
    /// Create a new tournament entry for the given zone and two competing clans.
    pub fn new(zone_id: u16, red_clan: u16, blue_clan: u16, duration_secs: u32) -> Self {
        Self {
            zone_id,
            clan_num: [red_clan, blue_clan],
            score_board: [0, 0],
            timer_secs: duration_secs,
            monument_killed: 0,
            out_timer: 0,
            is_attackable: true,
            is_started: true,
            is_finished: false,
        }
    }

    /// Whether the battle timer is still running.
    pub fn is_timer_running(&self) -> bool {
        self.is_started && self.timer_secs > 0
    }
}

/// In-memory registry of active tournament arenas, keyed by zone_id.
pub type TournamentRegistry = DashMap<u16, TournamentState>;

/// Create an empty tournament registry (used in `WorldState::new()`).
pub fn new_tournament_registry() -> TournamentRegistry {
    DashMap::new()
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Return `true` if `zone_id` is one of the six tournament arenas.
pub fn is_tournament_zone(zone_id: u16) -> bool {
    TOURNAMENT_ZONES.contains(&zone_id)
}

/// Current unix timestamp (seconds).
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ── Packet builders ────────────────────────────────────────────────────────

/// Build a `WIZ_BATTLE_EVENT (0x57)` tournament score update packet.
/// Packet layout:
/// ```text
/// u8  sub_opcode  = 0x12
/// u8  board_type  = 2
/// u16 red_score
/// u16 blue_score
/// u32 timer_secs
/// u8  monument_killed
/// ```
pub fn build_score_packet(state: &TournamentState) -> Packet {
    let mut pkt = Packet::new(Opcode::WizBattleEvent as u8);
    pkt.write_u8(BATTLE_EVENT_TOURNAMENT_SCORE); // 0x12
    pkt.write_u8(2); // board_type = 2
    pkt.write_u16(state.score_board[0]); // Red clan score
    pkt.write_u16(state.score_board[1]); // Blue clan score
    pkt.write_u32(state.timer_secs); // Remaining seconds
    pkt.write_u8(state.monument_killed); // Monument advantage
    pkt
}

/// Build a `WIZ_BIFROST (0x7B)` tournament timer packet.
/// Packet layout:
/// ```text
/// u8  sub_type   = 5
/// u16 timer_secs
/// ```
pub fn build_timer_packet(timer_secs: u32) -> Packet {
    let mut pkt = Packet::new(Opcode::WizBifrost as u8);
    pkt.write_u8(BIFROST_TOURNAMENT_TIMER); // sub_type = 5
    pkt.write_u16(timer_secs as u16); // Remaining seconds (u16 in C++)
    pkt
}

/// Build a `WIZ_KNIGHTS_PROCESS (0x6E)` clan-list packet for tournament arena login.
/// Sent to a player when they enter a tournament zone so their UI can show
/// both competing clans' names and emblems.
/// Packet layout:
/// ```text
/// SByte (size prefix)
/// u8  zone_id
/// u16 red_clan_id
/// u16 red_mark_version  (written twice per C++ source)
/// u16 red_mark_version
/// str red_clan_name      (written twice)
/// str red_clan_name
/// u16 blue_clan_id
/// u16 blue_mark_version
/// u16 blue_mark_version
/// str blue_clan_name
/// str blue_clan_name
/// ```
pub fn build_knights_vs_list_packet(
    state: &TournamentState,
    red_clan: &KnightsInfo,
    blue_clan: &KnightsInfo,
) -> Packet {
    let mut pkt = Packet::new(Opcode::WizKnightsProcess as u8);
    pkt.write_u8(KNIGHTS_VS_LIST_OPCODE); // sub-opcode 96

    // SByte prefix written inline (no separate length prefix in our Packet impl)
    pkt.write_u8(state.zone_id as u8);

    // Red clan — C++ uses SByte() so strings have u8 length prefix
    pkt.write_u16(red_clan.id);
    pkt.write_u16(red_clan.mark_version); // written twice per C++
    pkt.write_u16(red_clan.mark_version);
    pkt.write_sbyte_string(&red_clan.name); // written twice per C++
    pkt.write_sbyte_string(&red_clan.name);

    // Blue clan
    pkt.write_u16(blue_clan.id);
    pkt.write_u16(blue_clan.mark_version);
    pkt.write_u16(blue_clan.mark_version);
    pkt.write_sbyte_string(&blue_clan.name);
    pkt.write_sbyte_string(&blue_clan.name);

    pkt
}

/// Build a zone-change teleport packet to move a player to `dest_zone`.
pub fn build_zone_change_to_moradon(nation: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizZoneChange as u8);
    pkt.write_u8(3); // ZONE_CHANGE_TELEPORT
    pkt.write_u16(ZONE_MORADON);
    pkt.write_u16(0); // reserved
    pkt.write_u16(0); // x=0 → default spawn
    pkt.write_u16(0); // z=0 → default spawn
    pkt.write_u16(0); // reserved
    pkt.write_u8(nation);
    pkt.write_u16(0xFFFF);
    pkt
}

// ── Main tournament operations ─────────────────────────────────────────────

/// Called when a player kills an enemy in a tournament zone.
/// Increments the killer's clan score and broadcasts the updated scoreboard
/// and timer to all players in the arena.
///                in `TournamentSystem.cpp:383-440`.
/// # Arguments
/// - `world` — shared world state (for zone broadcast and clan lookup)
/// - `zone_id` — the tournament zone where the kill occurred
/// - `killer_clan_id` — clan ID of the player who scored the kill
pub fn register_kill(world: &WorldState, zone_id: u16, killer_clan_id: u16) {
    if !is_tournament_zone(zone_id) {
        return;
    }
    if killer_clan_id == 0 {
        return;
    }

    let mut scored = false;
    world.with_tournament(zone_id, |state| {
        if !state.is_started || state.timer_secs == 0 {
            return;
        }
        if killer_clan_id == state.clan_num[0] {
            state.score_board[0] = state.score_board[0].saturating_add(1);
            scored = true;
        } else if killer_clan_id == state.clan_num[1] {
            state.score_board[1] = state.score_board[1].saturating_add(1);
            scored = true;
        }
    });

    if scored {
        broadcast_score_and_timer(world, zone_id);
    }
}

/// Called when a tournament monument NPC is killed.
/// If the LOSING clan killed the monument, they receive a bonus equal to half
/// the score gap (bringing them closer). The scoreboard is rebroadcast.
///                in `TournamentSystem.cpp:443-539`.
/// # Arguments
/// - `world` — shared world state
/// - `zone_id` — the tournament zone containing the monument
/// - `killer_clan_id` — clan ID of the player who destroyed the monument
pub fn register_monument_kill(world: &WorldState, zone_id: u16, killer_clan_id: u16) {
    if !is_tournament_zone(zone_id) {
        return;
    }

    let mut updated = false;
    world.with_tournament(zone_id, |state| {
        if !state.is_started || state.timer_secs == 0 {
            return;
        }

        // Monument bonus only when scores differ
        if state.score_board[0] == state.score_board[1] {
            return;
        }

        let red_score = state.score_board[0];
        let blue_score = state.score_board[1];

        // C++ condition: killer is red clan AND red_clan_id < blue_clan_id → bonus if losing
        if killer_clan_id == state.clan_num[0]
            && state.clan_num[0] < state.clan_num[1]
            && red_score < blue_score
        {
            let half = (blue_score - red_score) / 2;
            state.score_board[0] = state.score_board[0].saturating_add(half);
            state.monument_killed = state.monument_killed.saturating_add(1);
            updated = true;
        }
        // Symmetrically for blue clan
        else if killer_clan_id == state.clan_num[1]
            && state.clan_num[1] < state.clan_num[0]
            && blue_score < red_score
        {
            let half = (red_score - blue_score) / 2;
            state.score_board[1] = state.score_board[1].saturating_add(half);
            state.monument_killed = state.monument_killed.saturating_add(1);
            updated = true;
        }
    });

    if updated {
        broadcast_score_and_timer(world, zone_id);
    }
}

/// Broadcast the current score + timer to all players in the tournament zone.
/// `UpdateClanTournamentScoreBoard` and `TournamentMonumentKillProcess`.
fn broadcast_score_and_timer(world: &WorldState, zone_id: u16) {
    let (score_pkt, timer_pkt) = world
        .with_tournament_snapshot(zone_id)
        .map(|state| {
            (
                build_score_packet(&state),
                build_timer_packet(state.timer_secs),
            )
        })
        .unwrap_or_else(|| {
            let dummy = TournamentState::new(zone_id, 0, 0, 0);
            (build_score_packet(&dummy), build_timer_packet(0))
        });

    let arc_score = Arc::new(score_pkt);
    let arc_timer = Arc::new(timer_pkt);
    let sessions = world.sessions_in_zone(zone_id);
    for sid in sessions {
        world.send_to_session_arc(sid, Arc::clone(&arc_score));
        world.send_to_session_arc(sid, Arc::clone(&arc_timer));
    }
}

/// Send the current tournament state (clan list + score + timer) to a single player.
/// Called when a player enters a tournament zone so their UI is immediately up-to-date.
///                `CKnightsManager::KnightsVsLoginList()` in `KnightsManager.cpp:176-220`.
pub fn send_state_to_player(world: &WorldState, sid: SessionId) {
    let pos = match world.with_session(sid, |h| h.position) {
        Some(p) => p,
        None => return,
    };

    if !is_tournament_zone(pos.zone_id) {
        return;
    }

    let snapshot = match world.with_tournament_snapshot(pos.zone_id) {
        Some(s) => s,
        None => return,
    };

    if !snapshot.is_started {
        return;
    }

    let red_clan = world.get_knights(snapshot.clan_num[0]);
    let blue_clan = world.get_knights(snapshot.clan_num[1]);

    if let (Some(red), Some(blue)) = (red_clan, blue_clan) {
        let list_pkt = build_knights_vs_list_packet(&snapshot, &red, &blue);
        world.send_to_session_owned(sid, list_pkt);
    }

    let timer_pkt = build_timer_packet(snapshot.timer_secs);
    world.send_to_session_owned(sid, timer_pkt);

    let score_pkt = build_score_packet(&snapshot);
    world.send_to_session_owned(sid, score_pkt);
}

/// GM command: start a new tournament in `zone_id` between two clans.
/// Validates zone, looks up both clans by name, creates the TournamentState,
/// and broadcasts the initial packets to all players in the zone.
/// fully implemented here based on the Close/Timer/ScoreBoard patterns).
/// # Arguments
/// - `world` — shared world state
/// - `zone_id` — arena zone (77/78/96-99)
/// - `clan_name_red` — name of the Red clan
/// - `clan_name_blue` — name of the Blue clan
/// - `duration_secs` — battle duration in seconds
/// Returns `Err` with a descriptive message if validation fails.
pub fn start_tournament(
    world: &Arc<WorldState>,
    zone_id: u16,
    clan_name_red: &str,
    clan_name_blue: &str,
    duration_secs: u32,
) -> Result<(), String> {
    // Validate zone
    if !is_tournament_zone(zone_id) {
        return Err(format!("Invalid tournament zone: {zone_id}"));
    }

    // Validate names
    if clan_name_red.is_empty() || clan_name_red.len() > 21 {
        return Err("Red clan name empty or too long (>21)".to_string());
    }
    if clan_name_blue.is_empty() || clan_name_blue.len() > 21 {
        return Err("Blue clan name empty or too long (>21)".to_string());
    }
    if clan_name_red.eq_ignore_ascii_case(clan_name_blue) {
        return Err("Red and Blue clan names must differ".to_string());
    }

    // Lookup clans by name
    let red_clan = world
        .find_knights_by_name(clan_name_red)
        .ok_or_else(|| format!("Clan not found: {clan_name_red}"))?;
    let blue_clan = world
        .find_knights_by_name(clan_name_blue)
        .ok_or_else(|| format!("Clan not found: {clan_name_blue}"))?;

    // Close any existing tournament in this zone first
    world.remove_tournament(zone_id);

    // Register new tournament
    let state = TournamentState::new(zone_id, red_clan.id, blue_clan.id, duration_secs);
    world.insert_tournament(state);

    debug!(
        "Tournament started: zone={} Red='{}' (id={}) Blue='{}' (id={}) duration={}s",
        zone_id, clan_name_red, red_clan.id, clan_name_blue, blue_clan.id, duration_secs
    );

    // Broadcast initial state to zone
    let snapshot = world
        .with_tournament_snapshot(zone_id)
        .unwrap_or_else(|| TournamentState::new(zone_id, red_clan.id, blue_clan.id, duration_secs));

    let list_pkt = build_knights_vs_list_packet(&snapshot, &red_clan, &blue_clan);
    let timer_pkt = build_timer_packet(snapshot.timer_secs);
    let score_pkt = build_score_packet(&snapshot);

    let arc_list = Arc::new(list_pkt);
    let arc_timer = Arc::new(timer_pkt);
    let arc_score = Arc::new(score_pkt);
    let sessions = world.sessions_in_zone(zone_id);
    for sid in sessions {
        world.send_to_session_arc(sid, Arc::clone(&arc_list));
        world.send_to_session_arc(sid, Arc::clone(&arc_timer));
        world.send_to_session_arc(sid, Arc::clone(&arc_score));
    }

    Ok(())
}

/// GM command: close/cancel a tournament in `zone_id`.
/// Kicks all players in the arena to Moradon and removes the tournament entry.
/// # Arguments
/// - `world` — shared world state
/// - `zone_id` — arena zone (77/78/96-99) to terminate
pub fn close_tournament(world: &Arc<WorldState>, zone_id: u16) {
    if !is_tournament_zone(zone_id) {
        debug!(
            "close_tournament: zone {} is not a tournament zone",
            zone_id
        );
        return;
    }

    // Kick all players in the zone to Moradon
    kick_zone_to_moradon(world, zone_id);

    // Remove from registry
    world.remove_tournament(zone_id);

    debug!("Tournament closed for zone={}", zone_id);
}

/// Tick function: called once per second from the game timer task.
/// Decrements each active tournament's timer, announces results when time
/// expires, and removes finished tournaments after the grace period.
pub fn tournament_tick(world: &Arc<WorldState>) {
    let now = now_secs();

    for zone_id in TOURNAMENT_ZONES {
        // Collect state snapshot without holding the registry lock
        let snapshot = match world.with_tournament_snapshot(zone_id) {
            Some(s) => s,
            None => continue,
        };

        if snapshot.is_started {
            // Battle phase: count down timer
            if !snapshot.is_finished && snapshot.timer_secs == 0 {
                // Battle just ended — determine winner and set out_timer
                handle_battle_end(world, zone_id, &snapshot);
                world.with_tournament(zone_id, |s| {
                    s.out_timer = now + TOURNAMENT_OUT_TIMER_SECS;
                    s.is_started = false;
                });
            } else if snapshot.timer_secs > 0 {
                world.with_tournament(zone_id, |s| {
                    s.timer_secs = s.timer_secs.saturating_sub(1);
                });
            }
        } else {
            // Grace period phase
            if snapshot.out_timer > 0 && now >= snapshot.out_timer && !snapshot.is_finished {
                world.with_tournament(zone_id, |s| {
                    s.is_finished = true;
                });
            }

            if snapshot.is_finished && snapshot.out_timer <= now {
                // Kick players and remove tournament entry
                kick_zone_to_moradon(world, zone_id);
                world.remove_tournament(zone_id);
                debug!("Tournament zone={} fully ended and removed", zone_id);
            }
        }
    }
}

/// Announce battle result via WAR_SYSTEM_CHAT and set the finished state.
/// `IDS_CLAN_WAR_DRAW_NOTICE` to all players server-wide.
fn handle_battle_end(world: &Arc<WorldState>, zone_id: u16, state: &TournamentState) {
    let red = state.score_board[0];
    let blue = state.score_board[1];

    let winner_clan_id: Option<u16> = if red > blue {
        Some(state.clan_num[0])
    } else if blue > red {
        Some(state.clan_num[1])
    } else {
        None // draw
    };

    match winner_clan_id {
        Some(cid) => {
            if let Some(clan) = world.get_knights(cid) {
                debug!(
                    "Tournament zone={} ended — winner: '{}' (id={}) score {}:{}",
                    zone_id, clan.name, cid, red, blue
                );
                // Broadcast IDS_CLAN_WAR_NOTICE: "Clan War is over. [ClanName] has won!"
                let message = format!("Clan War is over. {} has won!", clan.name);
                let pkt = crate::handler::chat::build_chat_packet(
                    WAR_SYSTEM_CHAT,
                    1,      // nation = ALL (C++ default bNation=1)
                    0xFFFF, // sender_id = -1 (C++ default int16 senderID=-1)
                    "",     // no sender name (system message)
                    &message,
                    0, // personal_rank
                    0, // authority = GM
                    0, // system_msg
                );
                world.broadcast_to_all(Arc::new(pkt), None);
            } else {
                debug!(
                    "Tournament zone={} ended — winner clan id={} (not found in registry) score {}:{}",
                    zone_id, cid, red, blue
                );
            }
        }
        None => {
            debug!(
                "Tournament zone={} ended — DRAW score {}:{}",
                zone_id, red, blue
            );
            // Broadcast IDS_CLAN_WAR_DRAW_NOTICE: draw announcement
            let message = "Clan War is over. The battle ended in a draw!";
            let pkt = crate::handler::chat::build_chat_packet(
                WAR_SYSTEM_CHAT,
                1,      // nation = ALL (C++ default bNation=1)
                0xFFFF, // sender_id = -1 (C++ default int16 senderID=-1)
                "",     // no sender name (system message)
                message,
                0, // personal_rank
                0, // authority = GM
                0, // system_msg
            );
            world.broadcast_to_all(Arc::new(pkt), None);
        }
    }
}

/// Kick all players in `zone_id` to Moradon (zone 21).
fn kick_zone_to_moradon(world: &Arc<WorldState>, zone_id: u16) {
    let sessions = world.sessions_in_zone(zone_id);
    for sid in sessions {
        let nation = world
            .with_session(sid, |h| h.character.as_ref().map(|c| c.nation))
            .flatten()
            .unwrap_or(1);

        let pkt = build_zone_change_to_moradon(nation);
        world.update_position(sid, ZONE_MORADON, 0.0, 0.0, 0.0);
        world.send_to_session_owned(sid, pkt);
    }
}

/// Validate that a player is allowed to be in the tournament zone.
/// Returns `true` if the player's clan is one of the two competing clans.
/// Returns `false` if they should be kicked (wrong clan / no active tournament).
///                and `TournamentSystem.cpp:454-469` (TournamentMonumentKillProcess).
pub fn is_player_allowed_in_zone(world: &WorldState, sid: SessionId, zone_id: u16) -> bool {
    if !is_tournament_zone(zone_id) {
        return true; // Not our concern
    }

    let clan_id = world.get_session_clan_id(sid);
    if clan_id == 0 {
        return false; // No clan → not allowed
    }

    world
        .with_tournament_snapshot(zone_id)
        .map(|state| clan_id == state.clan_num[0] || clan_id == state.clan_num[1])
        .unwrap_or(false) // No active tournament → not allowed
}

// ── Background tick task ────────────────────────────────────────────────────

/// Start the tournament timer background task (1-second tick).
/// Spawns a tokio task that calls [`tournament_tick`] every second,
/// processing tournament countdowns, battle end, and grace-period kicks.
/// Returns a `JoinHandle` so the caller can abort on shutdown.
pub fn start_tournament_tick_task(world: Arc<WorldState>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            tournament_tick(&world);
        }
    })
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── TournamentState unit tests ─────────────────────────────────────

    #[test]
    fn tournament_state_initial_values() {
        let state = TournamentState::new(77, 100, 200, 1800);
        assert_eq!(state.zone_id, 77);
        assert_eq!(state.clan_num, [100, 200]);
        assert_eq!(state.score_board, [0, 0]);
        assert_eq!(state.timer_secs, 1800);
        assert_eq!(state.monument_killed, 0);
        assert!(state.is_started);
        assert!(!state.is_finished);
        assert!(state.is_attackable);
    }

    #[test]
    fn is_timer_running_true_when_started_and_nonzero() {
        let state = TournamentState::new(77, 1, 2, 100);
        assert!(state.is_timer_running());
    }

    #[test]
    fn is_timer_running_false_when_timer_zero() {
        let mut state = TournamentState::new(77, 1, 2, 0);
        state.is_started = true;
        assert!(!state.is_timer_running());
    }

    #[test]
    fn is_timer_running_false_when_not_started() {
        let mut state = TournamentState::new(77, 1, 2, 100);
        state.is_started = false;
        assert!(!state.is_timer_running());
    }

    // ── is_tournament_zone tests ───────────────────────────────────────

    #[test]
    fn tournament_zone_ardream() {
        assert!(is_tournament_zone(ZONE_CLAN_WAR_ARDREAM));
    }

    #[test]
    fn tournament_zone_ronark() {
        assert!(is_tournament_zone(ZONE_CLAN_WAR_RONARK));
    }

    #[test]
    fn tournament_zone_party_vs_1() {
        assert!(is_tournament_zone(ZONE_PARTY_VS_1));
    }

    #[test]
    fn tournament_zone_party_vs_4() {
        assert!(is_tournament_zone(ZONE_PARTY_VS_4));
    }

    #[test]
    fn non_tournament_zone_moradon() {
        assert!(!is_tournament_zone(ZONE_MORADON));
    }

    #[test]
    fn non_tournament_zone_ardream_regular() {
        assert!(!is_tournament_zone(72)); // ZONE_ARDREAM regular battle zone
    }

    // ── Packet format tests ────────────────────────────────────────────

    #[test]
    fn score_packet_format() {
        let mut state = TournamentState::new(77, 10, 20, 600);
        state.score_board = [5, 3];
        state.monument_killed = 1;

        let pkt = build_score_packet(&state);
        assert_eq!(pkt.opcode, Opcode::WizBattleEvent as u8);
        // sub_opcode
        assert_eq!(pkt.data[0], BATTLE_EVENT_TOURNAMENT_SCORE); // 0x12
                                                                // board_type
        assert_eq!(pkt.data[1], 2);
        // red score (u16 LE)
        assert_eq!(u16::from_le_bytes([pkt.data[2], pkt.data[3]]), 5);
        // blue score (u16 LE)
        assert_eq!(u16::from_le_bytes([pkt.data[4], pkt.data[5]]), 3);
        // timer (u32 LE)
        assert_eq!(
            u32::from_le_bytes([pkt.data[6], pkt.data[7], pkt.data[8], pkt.data[9]]),
            600
        );
        // monument_killed
        assert_eq!(pkt.data[10], 1);
        // Total bytes: 1+1+2+2+4+1 = 11
        assert_eq!(pkt.data.len(), 11);
    }

    #[test]
    fn score_packet_zero_scores() {
        let state = TournamentState::new(96, 1, 2, 0);
        let pkt = build_score_packet(&state);
        assert_eq!(u16::from_le_bytes([pkt.data[2], pkt.data[3]]), 0);
        assert_eq!(u16::from_le_bytes([pkt.data[4], pkt.data[5]]), 0);
    }

    #[test]
    fn timer_packet_format() {
        let pkt = build_timer_packet(1200);
        assert_eq!(pkt.opcode, Opcode::WizBifrost as u8);
        assert_eq!(pkt.data[0], BIFROST_TOURNAMENT_TIMER); // sub_type=5
        assert_eq!(u16::from_le_bytes([pkt.data[1], pkt.data[2]]), 1200);
        assert_eq!(pkt.data.len(), 3);
    }

    #[test]
    fn timer_packet_truncates_to_u16() {
        // C++ uses u16 for timer in the packet even though internal storage is u32
        let pkt = build_timer_packet(70000); // > u16::MAX
        let val = u16::from_le_bytes([pkt.data[1], pkt.data[2]]);
        assert_eq!(val, 70000u32 as u16); // truncated to u16
    }

    #[test]
    fn zone_change_to_moradon_format() {
        let pkt = build_zone_change_to_moradon(1);
        assert_eq!(pkt.opcode, Opcode::WizZoneChange as u8);
        // type=3
        assert_eq!(pkt.data[0], 3);
        // zone_id = ZONE_MORADON = 21
        assert_eq!(u16::from_le_bytes([pkt.data[1], pkt.data[2]]), ZONE_MORADON);
        // x=0, z=0 (default spawn)
        assert_eq!(u16::from_le_bytes([pkt.data[5], pkt.data[6]]), 0);
        assert_eq!(u16::from_le_bytes([pkt.data[7], pkt.data[8]]), 0);
        // nation
        assert_eq!(pkt.data[11], 1);
        // 0xFFFF sentinel
        assert_eq!(u16::from_le_bytes([pkt.data[12], pkt.data[13]]), 0xFFFF);
    }

    // ── Monument bonus calculation tests ──────────────────────────────

    #[test]
    fn monument_kill_bonus_losing_red_clan() {
        // Red score < Blue score → red clan (losing) gets half the gap
        // Red: 10, Blue: 20 → gap=10, half=5, red gets +5 → red becomes 15
        let mut state = TournamentState::new(77, 1, 2, 100);
        state.score_board = [10, 20];
        state.clan_num = [1, 2]; // red id=1 < blue id=2

        // Simulate the monument kill logic for the losing (red) clan
        let red_score = state.score_board[0];
        let blue_score = state.score_board[1];
        let killer_clan_id = state.clan_num[0]; // red is killer

        if killer_clan_id == state.clan_num[0]
            && state.clan_num[0] < state.clan_num[1]
            && red_score < blue_score
        {
            let half = (blue_score - red_score) / 2;
            state.score_board[0] = state.score_board[0].saturating_add(half);
            state.monument_killed = state.monument_killed.saturating_add(1);
        }

        assert_eq!(state.score_board[0], 15);
        assert_eq!(state.score_board[1], 20);
        assert_eq!(state.monument_killed, 1);
    }

    #[test]
    fn monument_kill_no_bonus_when_winning() {
        // Red score > Blue score → red clan is winning, no monument bonus
        let mut state = TournamentState::new(77, 1, 2, 100);
        state.score_board = [20, 10];
        state.clan_num = [1, 2];

        let red_score = state.score_board[0];
        let blue_score = state.score_board[1];
        let killer_clan_id = state.clan_num[0]; // red is killer

        // C++ only grants bonus if score is lower (LOSING clan)
        if killer_clan_id == state.clan_num[0]
            && state.clan_num[0] < state.clan_num[1]
            && red_score < blue_score
        {
            let half = (blue_score - red_score) / 2;
            state.score_board[0] = state.score_board[0].saturating_add(half);
        }

        // Score unchanged (red was not losing)
        assert_eq!(state.score_board[0], 20);
    }

    #[test]
    fn monument_kill_no_bonus_when_draw() {
        let mut state = TournamentState::new(77, 1, 2, 100);
        state.score_board = [10, 10];

        // No bonus when scores are equal
        let red = state.score_board[0];
        let blue = state.score_board[1];
        assert_eq!(red, blue); // Both equal — no bonus
                               // score_board[0] intentionally unchanged
        assert_eq!(state.score_board, [10, 10]);
    }

    // ── Constants tests ────────────────────────────────────────────────

    #[test]
    fn tournament_zone_constants_match_cpp() {
        assert_eq!(ZONE_CLAN_WAR_ARDREAM, 77);
        assert_eq!(ZONE_CLAN_WAR_RONARK, 78);
        assert_eq!(ZONE_PARTY_VS_1, 96);
        assert_eq!(ZONE_PARTY_VS_2, 97);
        assert_eq!(ZONE_PARTY_VS_3, 98);
        assert_eq!(ZONE_PARTY_VS_4, 99);
    }

    #[test]
    fn knights_vs_list_opcode_value() {
        // C++ packets.h:644 — KNIGHTS_VS_LIST = 96
        assert_eq!(KNIGHTS_VS_LIST_OPCODE, 96);
    }

    #[test]
    fn battle_event_sub_opcode_value() {
        // C++ TournamentSystem.cpp:423 — uint8(0x12)
        assert_eq!(BATTLE_EVENT_TOURNAMENT_SCORE, 0x12);
    }

    #[test]
    fn bifrost_timer_sub_type_value() {
        // C++ TournamentSystem.cpp:434 — uint8(5)
        assert_eq!(BIFROST_TOURNAMENT_TIMER, 5);
    }

    #[test]
    fn out_timer_duration() {
        // C++ TournamentSystem.cpp:41 — UNIXTIME + 300
        assert_eq!(TOURNAMENT_OUT_TIMER_SECS, 300);
    }

    #[test]
    fn all_six_zones_covered() {
        assert_eq!(TOURNAMENT_ZONES.len(), 6);
        for z in TOURNAMENT_ZONES {
            assert!(
                is_tournament_zone(z),
                "zone {z} should be a tournament zone"
            );
        }
    }

    // ── start_tournament validation tests ─────────────────────────────

    #[test]
    fn start_tournament_rejects_invalid_zone() {
        let world = Arc::new(WorldState::new());
        let result = start_tournament(&world, 21, "ClanA", "ClanB", 1800);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid tournament zone"));
    }

    #[test]
    fn start_tournament_rejects_empty_name() {
        let world = Arc::new(WorldState::new());
        let result = start_tournament(&world, 77, "", "ClanB", 1800);
        assert!(result.is_err());
    }

    #[test]
    fn start_tournament_rejects_long_name() {
        let world = Arc::new(WorldState::new());
        let long_name = "A".repeat(22); // > 21 chars
        let result = start_tournament(&world, 77, &long_name, "ClanB", 1800);
        assert!(result.is_err());
    }

    #[test]
    fn start_tournament_rejects_same_names() {
        let world = Arc::new(WorldState::new());
        let result = start_tournament(&world, 77, "ClanA", "ClanA", 1800);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must differ"));
    }

    #[test]
    fn start_tournament_rejects_unknown_clan() {
        let world = Arc::new(WorldState::new());
        // No clans registered → should fail with "Clan not found"
        let result = start_tournament(&world, 77, "GhostClan", "ClanB", 1800);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    // ── register_kill integration tests ─────────────────────────────

    #[test]
    fn register_kill_increments_red_score() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.score_board = [3, 5];
        world.insert_tournament(state);

        register_kill(&world, 77, 100); // red clan kill
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board[0], 4);
        assert_eq!(snap.score_board[1], 5);
    }

    #[test]
    fn register_kill_increments_blue_score() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.score_board = [3, 5];
        world.insert_tournament(state);

        register_kill(&world, 77, 200); // blue clan kill
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board[0], 3);
        assert_eq!(snap.score_board[1], 6);
    }

    #[test]
    fn register_kill_ignores_non_tournament_zone() {
        let world = WorldState::new();
        // No tournament in zone 21
        register_kill(&world, 21, 100); // should not panic
    }

    #[test]
    fn register_kill_ignores_zero_clan() {
        let world = WorldState::new();
        let state = TournamentState::new(77, 100, 200, 600);
        world.insert_tournament(state);

        register_kill(&world, 77, 0); // no clan
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board, [0, 0]); // unchanged
    }

    #[test]
    fn register_kill_ignores_unrelated_clan() {
        let world = WorldState::new();
        let state = TournamentState::new(77, 100, 200, 600);
        world.insert_tournament(state);

        register_kill(&world, 77, 999); // not in tournament
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board, [0, 0]); // unchanged
    }

    // ── register_monument_kill integration tests ────────────────────

    #[test]
    fn register_monument_kill_bonus_losing_red() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.score_board = [5, 15]; // red losing
        world.insert_tournament(state);

        register_monument_kill(&world, 77, 100); // red kills monument
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board[0], 10); // 5 + (15-5)/2 = 10
        assert_eq!(snap.score_board[1], 15);
        assert_eq!(snap.monument_killed, 1);
    }

    #[test]
    fn register_monument_kill_no_bonus_winning() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.score_board = [15, 5]; // red winning
        world.insert_tournament(state);

        register_monument_kill(&world, 77, 100); // red kills but winning
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board, [15, 5]); // unchanged
        assert_eq!(snap.monument_killed, 0);
    }

    #[test]
    fn register_monument_kill_no_bonus_tied() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.score_board = [10, 10]; // tied
        world.insert_tournament(state);

        register_monument_kill(&world, 77, 100);
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board, [10, 10]); // unchanged
    }

    #[test]
    fn register_monument_kill_not_started() {
        let world = WorldState::new();
        let mut state = TournamentState::new(77, 100, 200, 600);
        state.is_started = false;
        state.score_board = [5, 15];
        world.insert_tournament(state);

        register_monument_kill(&world, 77, 100);
        let snap = world.with_tournament_snapshot(77).unwrap();
        assert_eq!(snap.score_board, [5, 15]); // unchanged
    }
}
