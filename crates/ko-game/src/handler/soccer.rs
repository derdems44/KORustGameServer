//! Soccer Event handler — Temple Soccer system.
//!
//! C++ Reference: `KOOriginalGameServer/GameServer/SoccerSystem.cpp` (404 lines)
//!
//! The soccer event is a per-zone in-memory event in Moradon zones (21-25).
//! Two teams (Red, Blue) of up to 11 players each compete on a field.
//! A ball NPC is tracked for goal detection via geometric collision.
//!
//! ## Flow
//! 1. Players join via `isEventSoccerMember` (team selection + zone check)
//! 2. Any member can start the match via `isEventSoccerStard` (both teams need 1+)
//! 3. Timer ticks once per second for 600s; ball NPC checked for goals
//! 4. At time=1, match ends, players teleported out, results sent
//! 5. 10s cooldown, then room resets
//!
//! ## Packet Format
//! Soccer event packets use `WIZ_MINING` opcode with first byte `0x10` (16):
//! - `0x10 0x01 socket_id(u32) team(i8) team(i8) blue_goals(u8) red_goals(u8)` — goal scored
//! - `0x10 0x02 timer(u16)` — timer update / join confirmation
//! - `0x10 0x04 winner(u8) blue_goals(u8) red_goals(u8)` — match end result
//!
//! The client kick action uses `WIZ_MINING` sub-opcode `MiningSoccer(10)`.

use std::collections::HashMap;
use std::sync::Arc;

use ko_protocol::{Opcode, Packet};

use crate::world::{ZONE_MORADON, ZONE_MORADON2, ZONE_MORADON3, ZONE_MORADON4, ZONE_MORADON5};

// ── Team Colour Constants ──────────────────────────────────────────────
// C++ Reference: `User.h:103-109` — `enum TeamColour`

/// No team assigned.
pub const TEAM_COLOUR_NONE: u8 = 0;
/// Blue team.
pub const TEAM_COLOUR_BLUE: u8 = 1;
/// Red team.
pub const TEAM_COLOUR_RED: u8 = 2;
/// Ball is outside the field boundary.
pub const TEAM_COLOUR_OUTSIDE: u8 = 3;
/// Ball is outside the Moradon zone entirely.
pub const TEAM_COLOUR_MAP: u8 = 4;

// ── Geometry Constants ─────────────────────────────────────────────────
// C++ Reference: `SoccerSystem.cpp:166-178`

/// Soccer field boundary (X axis).
const FIELD_X_MIN: f32 = 644.0;
const FIELD_X_MAX: f32 = 699.0;
/// Soccer field boundary (Z axis).
const FIELD_Z_MIN: f32 = 120.0;
const FIELD_Z_MAX: f32 = 200.0;

/// Red goal zone (ball entering here = Blue team scores).
/// C++ Reference: `SoccerSystem.cpp:176`
const RED_GOAL_X_MIN: f32 = 661.0;
const RED_GOAL_X_MAX: f32 = 681.0;
const RED_GOAL_Z_MIN: f32 = 108.0;
const RED_GOAL_Z_MAX: f32 = 120.0;

/// Blue goal zone (ball entering here = Red team scores).
/// C++ Reference: `SoccerSystem.cpp:178`
const BLUE_GOAL_X_MIN: f32 = 661.0;
const BLUE_GOAL_X_MAX: f32 = 681.0;
const BLUE_GOAL_Z_MIN: f32 = 199.0;
const BLUE_GOAL_Z_MAX: f32 = 208.0;

/// Ball reset position (center of the field).
/// C++ Reference: `SoccerSystem.cpp:255`
pub const BALL_CENTER_X: f32 = 672.0;
pub const BALL_CENTER_Z: f32 = 160.0;

/// Default spawn positions for players joining with (0,0).
/// C++ Reference: `SoccerSystem.cpp:57-60`
const BLUE_SPAWN_X: f32 = 672.0;
const BLUE_SPAWN_Z: f32 = 166.0;
const RED_SPAWN_X: f32 = 672.0;
const RED_SPAWN_Z: f32 = 154.0;

/// End-of-match teleport positions.
/// C++ Reference: `SoccerSystem.cpp:281-289`
const BLUE_END_X: f32 = 639.0;
const BLUE_END_Z: f32 = 194.0;
const RED_END_X: f32 = 703.0;
const RED_END_Z: f32 = 127.0;

/// Maximum players per team.
/// C++ Reference: `SoccerSystem.cpp:11-16`
const MAX_TEAM_SIZE: u8 = 11;
/// Maximum total players (both teams).
const MAX_TOTAL_PLAYERS: u8 = 22;

/// Match duration in seconds.
/// C++ Reference: `SoccerSystem.cpp:90`
const MATCH_DURATION: u16 = 600;

/// Cooldown ticks after match ends before room resets.
/// C++ Reference: `SoccerSystem.cpp:308`
const COOLDOWN_TICKS: u16 = 10;

/// Soccer event sub-opcode prefix byte in WIZ_MINING packets.
/// C++ Reference: `SoccerSystem.cpp:51` — `Packet result(WIZ_MINING); result << uint8(16)`
const SOCCER_EVENT_SUB: u8 = 0x10;

// ── Per-User Tracking ──────────────────────────────────────────────────

/// Per-user soccer event state.
///
/// C++ Reference: `_SOCCER_STARTED_EVENT_USER` in `GameDefine.h:3495`
#[derive(Debug, Clone)]
pub struct SoccerUser {
    /// Character name.
    pub name: String,
    /// Whether a prize has been given to this user.
    pub prize_given: bool,
    /// Whether the user logged out during the event.
    pub logged_out: bool,
    /// Which team the user is on (TEAM_COLOUR_BLUE or TEAM_COLOUR_RED).
    pub team: u8,
}

// ── Per-Zone Room State ────────────────────────────────────────────────

/// Per-zone soccer event room state.
///
/// C++ Reference: `_SOCCER_STATUS_INFO` in `GameDefine.h:3513`
#[derive(Debug, Clone)]
pub struct SoccerRoom {
    /// Whether the match is currently active.
    pub active: bool,
    /// Whether the cooldown timer is running (post-match).
    pub timer_flag: bool,
    /// Socket ID associated with the soccer ball NPC (-1 = none).
    pub socket_id: i16,
    /// Cooldown countdown (ticks remaining after match ends).
    pub cooldown_ticks: u16,
    /// Match timer (counts down from 600 to 0).
    pub match_time: u16,
    /// Number of players on the red team.
    pub red_count: u8,
    /// Number of players on the blue team.
    pub blue_count: u8,
    /// Blue team goal count.
    pub blue_goals: u8,
    /// Red team goal count.
    pub red_goals: u8,
    /// Ball NPC ID (-1 = none).
    pub ball_npc_id: i16,
    /// Registered users keyed by character name.
    pub users: HashMap<String, SoccerUser>,
}

impl Default for SoccerRoom {
    fn default() -> Self {
        Self {
            active: false,
            timer_flag: false,
            socket_id: -1,
            cooldown_ticks: 0,
            match_time: 0,
            red_count: 0,
            blue_count: 0,
            blue_goals: 0,
            red_goals: 0,
            ball_npc_id: -1,
            users: HashMap::new(),
        }
    }
}

impl SoccerRoom {
    /// Total number of registered users.
    ///
    /// C++ Reference: `_SOCCER_STATUS_INFO::GetRoomTotalUserCount()`
    pub fn total_user_count(&self) -> u8 {
        self.users.len() as u8
    }

    /// Whether the match is currently active.
    ///
    /// C++ Reference: `_SOCCER_STATUS_INFO::isSoccerAktive()`
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Whether the cooldown timer is running.
    ///
    /// C++ Reference: `_SOCCER_STATUS_INFO::isSoccerTime()`
    pub fn is_cooldown(&self) -> bool {
        self.timer_flag
    }

    /// Reset the room to its initial state.
    ///
    /// C++ Reference: `_SOCCER_STATUS_INFO::Clean()`
    pub fn clean(&mut self) {
        self.users.clear();
        self.active = false;
        self.timer_flag = false;
        self.socket_id = -1;
        self.cooldown_ticks = 0;
        self.match_time = 0;
        self.red_count = 0;
        self.blue_count = 0;
        self.blue_goals = 0;
        self.red_goals = 0;
    }
}

// ── Global Soccer State ────────────────────────────────────────────────

/// Global soccer event state holding per-zone rooms.
///
/// C++ Reference: `CGameServerDlg::m_TempleSoccerEventRoomList`
#[derive(Debug, Clone)]
pub struct SoccerState {
    /// Per-zone soccer rooms keyed by zone ID (21-25).
    pub rooms: HashMap<u16, SoccerRoom>,
}

impl Default for SoccerState {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        for zone in [
            ZONE_MORADON,
            ZONE_MORADON2,
            ZONE_MORADON3,
            ZONE_MORADON4,
            ZONE_MORADON5,
        ] {
            rooms.insert(zone, SoccerRoom::default());
        }
        Self { rooms }
    }
}

impl SoccerState {
    /// Get a mutable reference to a room by zone ID.
    pub fn get_room_mut(&mut self, zone_id: u16) -> Option<&mut SoccerRoom> {
        self.rooms.get_mut(&zone_id)
    }

    /// Get an immutable reference to a room by zone ID.
    pub fn get_room(&self, zone_id: u16) -> Option<&SoccerRoom> {
        self.rooms.get(&zone_id)
    }
}

/// Thread-safe soccer state wrapper using `parking_lot::RwLock`.
pub type SharedSoccerState = Arc<parking_lot::RwLock<SoccerState>>;

/// Create a new shared soccer state.
pub fn new_soccer_state() -> SharedSoccerState {
    Arc::new(parking_lot::RwLock::new(SoccerState::default()))
}

// ── Zone Helpers ───────────────────────────────────────────────────────

/// Check whether a zone is a Moradon zone (neutral zone for soccer).
///
/// C++ Reference: `SoccerSystem.cpp:5` — `GetZoneID() >= ZONE_MORADON && GetZoneID() <= ZONE_MORADON5`
pub fn is_moradon_zone(zone_id: u16) -> bool {
    (ZONE_MORADON..=ZONE_MORADON5).contains(&zone_id)
}

// ── Geometry Helpers ───────────────────────────────────────────────────

/// Check whether a position is inside the soccer field.
///
/// C++ Reference: `CUser::isInSoccerEvent()` in `SoccerSystem.cpp:166-168`
pub fn is_in_field(x: f32, z: f32) -> bool {
    x > FIELD_X_MIN && x < FIELD_X_MAX && z > FIELD_Z_MIN && z < FIELD_Z_MAX
}

/// Check the ball NPC's position and return which zone it is in.
///
/// C++ Reference: `CNpc::isInSoccerEvent()` in `SoccerSystem.cpp:170-193`
///
/// Returns:
/// - `TEAM_COLOUR_BLUE` if ball is in blue goal zone (red team scores)
/// - `TEAM_COLOUR_RED` if ball is in red goal zone (blue team scores)
/// - `TEAM_COLOUR_OUTSIDE` if ball is outside the field entirely
/// - `TEAM_COLOUR_NONE` if ball is on the field (no event)
/// - `TEAM_COLOUR_MAP` if not in a Moradon zone
pub fn check_ball_position(zone_id: u16, x: f32, z: f32) -> u8 {
    if !is_moradon_zone(zone_id) {
        return TEAM_COLOUR_MAP;
    }

    let in_blue_goal =
        x > BLUE_GOAL_X_MIN && x < BLUE_GOAL_X_MAX && z > BLUE_GOAL_Z_MIN && z < BLUE_GOAL_Z_MAX;
    let in_red_goal =
        x > RED_GOAL_X_MIN && x < RED_GOAL_X_MAX && z > RED_GOAL_Z_MIN && z < RED_GOAL_Z_MAX;
    let in_field = is_in_field(x, z);

    if in_blue_goal {
        return TEAM_COLOUR_BLUE;
    }
    if in_red_goal {
        return TEAM_COLOUR_RED;
    }
    if !in_field {
        return TEAM_COLOUR_OUTSIDE;
    }

    TEAM_COLOUR_NONE
}

// ── Join Event ─────────────────────────────────────────────────────────

/// Result of attempting to join the soccer event.
#[derive(Debug, PartialEq)]
pub enum JoinResult {
    /// Successfully joined the event.
    Ok {
        /// Spawn X position.
        spawn_x: f32,
        /// Spawn Z position.
        spawn_z: f32,
        /// Current match timer value.
        match_time: u16,
    },
    /// Not in a Moradon zone.
    NotMoradon,
    /// Invalid team colour.
    InvalidTeam,
    /// Team is full (11 max).
    TeamFull,
    /// Room is full (22 max).
    RoomFull,
    /// Already registered in the event.
    AlreadyJoined,
    /// Zone has no soccer room.
    NoRoom,
}

/// Attempt to register a user into the soccer event for a zone.
///
/// C++ Reference: `CUser::isEventSoccerMember()` in `SoccerSystem.cpp:3-67`
///
/// Returns a `JoinResult` indicating success or failure.
pub fn join_event(
    state: &mut SoccerState,
    zone_id: u16,
    player_name: &str,
    team: u8,
    x: f32,
    z: f32,
) -> JoinResult {
    if !is_moradon_zone(zone_id) {
        return JoinResult::NotMoradon;
    }

    // Team must be Blue(1) or Red(2).
    // C++ Reference: `SoccerSystem.cpp:19-21`
    if !(TEAM_COLOUR_BLUE..=TEAM_COLOUR_RED).contains(&team) {
        return JoinResult::InvalidTeam;
    }

    let room = match state.get_room_mut(zone_id) {
        Some(r) => r,
        None => return JoinResult::NoRoom,
    };

    // Check team capacity.
    if team == TEAM_COLOUR_RED && room.red_count >= MAX_TEAM_SIZE {
        return JoinResult::TeamFull;
    }
    if team == TEAM_COLOUR_BLUE && room.blue_count >= MAX_TEAM_SIZE {
        return JoinResult::TeamFull;
    }

    // Check total capacity.
    if room.total_user_count() >= MAX_TOTAL_PLAYERS {
        return JoinResult::RoomFull;
    }

    // Check if already registered.
    if room.users.contains_key(player_name) {
        return JoinResult::AlreadyJoined;
    }

    // Register the user.
    room.users.insert(
        player_name.to_string(),
        SoccerUser {
            name: player_name.to_string(),
            prize_given: false,
            logged_out: false,
            team,
        },
    );

    // Increment team count.
    if team == TEAM_COLOUR_BLUE {
        room.blue_count += 1;
    } else {
        room.red_count += 1;
    }

    // Determine spawn position: use defaults if (0,0) was passed.
    // C++ Reference: `SoccerSystem.cpp:55-61`
    let (spawn_x, spawn_z) = if x == 0.0 && z == 0.0 {
        if team == TEAM_COLOUR_BLUE {
            (BLUE_SPAWN_X, BLUE_SPAWN_Z)
        } else {
            (RED_SPAWN_X, RED_SPAWN_Z)
        }
    } else {
        (x, z)
    };

    JoinResult::Ok {
        spawn_x,
        spawn_z,
        match_time: room.match_time,
    }
}

// ── Start Match ────────────────────────────────────────────────────────

/// Attempt to start the soccer match in a zone.
///
/// C++ Reference: `CUser::isEventSoccerStard()` in `SoccerSystem.cpp:69-93`
///
/// Returns `true` if the match was started, `false` if conditions not met.
pub fn start_match(state: &mut SoccerState, zone_id: u16, player_name: &str) -> bool {
    let room = match state.get_room_mut(zone_id) {
        Some(r) => r,
        None => return false,
    };

    // Cannot start if already active.
    if room.is_active() {
        return false;
    }

    // Must be a registered user.
    if !room.users.contains_key(player_name) {
        return false;
    }

    // Both teams must have at least 1 player.
    if room.blue_count == 0 || room.red_count == 0 {
        return false;
    }

    room.match_time = MATCH_DURATION;
    room.active = true;
    true
}

// ── Timer Tick ─────────────────────────────────────────────────────────

/// Result of a single timer tick.
#[derive(Debug, PartialEq)]
pub enum TickResult {
    /// Match just started (time == 600): send timer to all users.
    MatchStart {
        /// The current match time.
        time: u16,
    },
    /// Ball needs position check (1 < time < 600).
    BallCheck,
    /// Match just ended (time == 1).
    MatchEnd {
        /// Blue team goals.
        blue_goals: u8,
        /// Red team goals.
        red_goals: u8,
    },
    /// Room is in cooldown phase.
    Cooldown {
        /// Remaining cooldown ticks.
        remaining: u16,
    },
    /// Cooldown just finished — room should be cleaned.
    CooldownDone,
    /// Nothing to do (room not active, not in cooldown).
    Idle,
}

/// Process a single timer tick for a soccer room in a given zone.
///
/// C++ Reference: `CGameServerDlg::TempleSoccerEventTimer()` in `SoccerSystem.cpp:197-327`
///
/// This should be called once per second for each Moradon zone.
pub fn timer_tick(room: &mut SoccerRoom) -> TickResult {
    if room.is_active() {
        let result = if room.match_time == MATCH_DURATION {
            TickResult::MatchStart {
                time: room.match_time,
            }
        } else if room.match_time > 1 {
            TickResult::BallCheck
        } else {
            // time == 1: match ends
            let result = TickResult::MatchEnd {
                blue_goals: room.blue_goals,
                red_goals: room.red_goals,
            };

            // Transition to cooldown.
            // C++ Reference: `SoccerSystem.cpp:304-309`
            room.timer_flag = true;
            room.active = false;
            room.cooldown_ticks = COOLDOWN_TICKS;

            result
        };

        // Decrement the match timer.
        // C++ Reference: `SoccerSystem.cpp:312-313`
        if room.match_time > 0 {
            room.match_time -= 1;
        }

        result
    } else if room.is_cooldown() {
        // In cooldown phase.
        // C++ Reference: `SoccerSystem.cpp:315-325`
        if room.cooldown_ticks == 1 {
            room.cooldown_ticks = 0;
            TickResult::CooldownDone
        } else {
            let remaining = room.cooldown_ticks;
            if room.cooldown_ticks > 0 {
                room.cooldown_ticks -= 1;
            }
            TickResult::Cooldown { remaining }
        }
    } else {
        TickResult::Idle
    }
}

// ── Goal Scored ────────────────────────────────────────────────────────

/// Record a goal scored and return the updated goal counts.
///
/// C++ Reference: `SoccerSystem.cpp:244-256`
///
/// When the ball enters a goal zone:
/// - Ball in Red goal zone (`TEAM_COLOUR_RED`) → Blue team scores
/// - Ball in Blue goal zone (`TEAM_COLOUR_BLUE`) → Red team scores
///
/// Returns `(blue_goals, red_goals)` after the update.
pub fn record_goal(room: &mut SoccerRoom, goal_zone: u8) -> (u8, u8) {
    // C++ Reference: `SoccerSystem.cpp:244-249`
    // Note: C++ logic is counterintuitive — if ball is in RED zone, BLUE scored
    if goal_zone == TEAM_COLOUR_RED {
        room.blue_goals += 1;
    } else if goal_zone == TEAM_COLOUR_BLUE {
        room.red_goals += 1;
    }
    (room.blue_goals, room.red_goals)
}

// ── End Match ──────────────────────────────────────────────────────────

/// Determine the winning team from goal counts.
///
/// C++ Reference: `CUser::isEventSoccerEnd()` in `SoccerSystem.cpp:95-126`
///
/// Returns the winning team colour, or `TEAM_COLOUR_NONE` for a draw.
pub fn determine_winner(blue_goals: u8, red_goals: u8) -> u8 {
    if red_goals > blue_goals {
        TEAM_COLOUR_RED
    } else if blue_goals > red_goals {
        TEAM_COLOUR_BLUE
    } else {
        TEAM_COLOUR_NONE
    }
}

/// Get end-of-match teleport position for a given team.
///
/// C++ Reference: `SoccerSystem.cpp:279-289`
pub fn end_teleport_position(team: u8) -> (f32, f32) {
    match team {
        TEAM_COLOUR_BLUE => (BLUE_END_X, BLUE_END_Z),
        TEAM_COLOUR_RED => (RED_END_X, RED_END_Z),
        _ => (BALL_CENTER_X, BALL_CENTER_Z),
    }
}

/// Remove a user from the soccer event and decrement team counts.
///
/// C++ Reference: `CUser::isEventSoccerEnd()` lines 116-122 +
///                `CUser::isEventSoccerUserRemoved()` in `SoccerSystem.cpp:145-154`
pub fn remove_user(room: &mut SoccerRoom, player_name: &str) {
    if let Some(user) = room.users.remove(player_name) {
        if user.team == TEAM_COLOUR_BLUE && room.blue_count > 0 {
            room.blue_count -= 1;
        } else if user.team == TEAM_COLOUR_RED && room.red_count > 0 {
            room.red_count -= 1;
        }
    }
}

/// Check if a player is registered in the soccer event for a given zone.
///
/// C++ Reference: `CUser::isSoccerEventUser()` in `SoccerSystem.cpp:128-143`
pub fn is_soccer_user(state: &SoccerState, zone_id: u16, player_name: &str) -> bool {
    state
        .get_room(zone_id)
        .is_some_and(|r| r.users.contains_key(player_name))
}

/// Check if a player is in the soccer event and on the field.
///
/// C++ Reference: `CUser::isInSoccerEvent()` in `SoccerSystem.cpp:156-168`
pub fn is_player_in_soccer(
    state: &SoccerState,
    zone_id: u16,
    player_name: &str,
    x: f32,
    z: f32,
) -> bool {
    if !is_moradon_zone(zone_id) {
        return false;
    }
    if !is_soccer_user(state, zone_id, player_name) {
        return false;
    }
    is_in_field(x, z)
}

// ── Packet Builders ────────────────────────────────────────────────────

/// Build the timer/join confirmation packet.
///
/// C++ Reference: `SoccerSystem.cpp:51-53`
/// Format: `WIZ_MINING(0x10, 0x02, timer(u16))`
pub fn build_timer_packet(timer: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizMining as u8);
    pkt.write_u8(SOCCER_EVENT_SUB);
    pkt.write_u8(0x02);
    pkt.write_u16(timer);
    pkt
}

/// Build the goal scored notification packet.
///
/// C++ Reference: `SoccerSystem.cpp:369-374`
/// Format: `WIZ_MINING(0x10, 0x01, socket_id(u32), goal_zone(i8), goal_zone(i8), blue_goals(u8), red_goals(u8))`
pub fn build_goal_packet(socket_id: i16, goal_zone: u8, blue_goals: u8, red_goals: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizMining as u8);
    pkt.write_u8(SOCCER_EVENT_SUB);
    pkt.write_u8(0x01);
    pkt.write_u32(socket_id as u32);
    pkt.write_i8(goal_zone as i8);
    pkt.write_i8(goal_zone as i8);
    pkt.write_u8(blue_goals);
    pkt.write_u8(red_goals);
    pkt
}

/// Build the match end result packet.
///
/// C++ Reference: `SoccerSystem.cpp:114`
/// Format: `WIZ_MINING(0x10, 0x04, winner(u8), blue_goals(u8), red_goals(u8))`
pub fn build_end_packet(winner: u8, blue_goals: u8, red_goals: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizMining as u8);
    pkt.write_u8(SOCCER_EVENT_SUB);
    pkt.write_u8(0x04);
    pkt.write_u8(winner);
    pkt.write_u8(blue_goals);
    pkt.write_u8(red_goals);
    pkt
}

// ── Handle Soccer Kick (MiningSoccer=10) ───────────────────────────────

/// Handle the soccer kick action (client sends WIZ_MINING sub-opcode 10).
///
/// C++ Reference: `CUser::HandleSoccer()` in `SoccerSystem.cpp:381-404`
///
/// This sets the player as "mining" (which in this context means kicking
/// the ball) and broadcasts the action to the region.
///
/// Returns the packet to send (either success broadcast or error to self).
pub fn build_kick_response(session_id: u16, already_mining: bool) -> (Packet, bool) {
    let mut pkt = Packet::new(Opcode::WizMining as u8);
    pkt.write_u8(SOCCER_EVENT_SUB); // MiningSoccer = 10, but C++ uses the same code path

    let result_code: u16 = if already_mining {
        2 // MiningResultMiningAlready
    } else {
        1 // MiningResultSuccess
    };

    pkt.write_u16(result_code);

    let broadcast = !already_mining;
    if broadcast {
        pkt.write_u32(session_id as u32);
    }

    (pkt, broadcast)
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_moradon_zone() {
        assert!(is_moradon_zone(21));
        assert!(is_moradon_zone(22));
        assert!(is_moradon_zone(23));
        assert!(is_moradon_zone(24));
        assert!(is_moradon_zone(25));
        assert!(!is_moradon_zone(20));
        assert!(!is_moradon_zone(26));
        assert!(!is_moradon_zone(1));
    }

    #[test]
    fn test_is_in_field() {
        // Center of the field.
        assert!(is_in_field(672.0, 160.0));
        // Just inside boundaries.
        assert!(is_in_field(645.0, 121.0));
        assert!(is_in_field(698.0, 199.0));
        // On the boundary (exclusive — C++ uses strict > and <).
        assert!(!is_in_field(644.0, 160.0));
        assert!(!is_in_field(699.0, 160.0));
        assert!(!is_in_field(672.0, 120.0));
        assert!(!is_in_field(672.0, 200.0));
        // Outside.
        assert!(!is_in_field(600.0, 160.0));
        assert!(!is_in_field(672.0, 300.0));
    }

    #[test]
    fn test_check_ball_position_red_goal() {
        // Ball in red goal zone (x: 661-681, z: 108-120).
        // Blue team scores when ball enters red goal.
        let result = check_ball_position(21, 670.0, 115.0);
        assert_eq!(result, TEAM_COLOUR_RED);
    }

    #[test]
    fn test_check_ball_position_blue_goal() {
        // Ball in blue goal zone (x: 661-681, z: 199-208).
        // Red team scores when ball enters blue goal.
        let result = check_ball_position(21, 670.0, 203.0);
        assert_eq!(result, TEAM_COLOUR_BLUE);
    }

    #[test]
    fn test_check_ball_position_on_field() {
        let result = check_ball_position(21, 672.0, 160.0);
        assert_eq!(result, TEAM_COLOUR_NONE);
    }

    #[test]
    fn test_check_ball_position_outside_field() {
        let result = check_ball_position(21, 600.0, 160.0);
        assert_eq!(result, TEAM_COLOUR_OUTSIDE);
    }

    #[test]
    fn test_check_ball_position_not_moradon() {
        let result = check_ball_position(1, 672.0, 160.0);
        assert_eq!(result, TEAM_COLOUR_MAP);
    }

    #[test]
    fn test_join_event_basic() {
        let mut state = SoccerState::default();
        let result = join_event(&mut state, 21, "player1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        assert!(matches!(
            result,
            JoinResult::Ok {
                spawn_x: 672.0,
                spawn_z: 166.0,
                ..
            }
        ));

        let room = state.get_room(21).unwrap();
        assert_eq!(room.blue_count, 1);
        assert_eq!(room.red_count, 0);
        assert_eq!(room.total_user_count(), 1);
    }

    #[test]
    fn test_join_event_red_team() {
        let mut state = SoccerState::default();
        let result = join_event(&mut state, 21, "player1", TEAM_COLOUR_RED, 0.0, 0.0);
        assert!(matches!(
            result,
            JoinResult::Ok {
                spawn_x: 672.0,
                spawn_z: 154.0,
                ..
            }
        ));
        let room = state.get_room(21).unwrap();
        assert_eq!(room.red_count, 1);
    }

    #[test]
    fn test_join_event_custom_position() {
        let mut state = SoccerState::default();
        let result = join_event(&mut state, 21, "player1", TEAM_COLOUR_BLUE, 660.0, 170.0);
        assert!(matches!(
            result,
            JoinResult::Ok {
                spawn_x: 660.0,
                spawn_z: 170.0,
                ..
            }
        ));
    }

    #[test]
    fn test_join_event_not_moradon() {
        let mut state = SoccerState::default();
        let result = join_event(&mut state, 1, "player1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        assert_eq!(result, JoinResult::NotMoradon);
    }

    #[test]
    fn test_join_event_invalid_team() {
        let mut state = SoccerState::default();
        let result = join_event(&mut state, 21, "player1", 0, 0.0, 0.0);
        assert_eq!(result, JoinResult::InvalidTeam);
        let result = join_event(&mut state, 21, "player1", 3, 0.0, 0.0);
        assert_eq!(result, JoinResult::InvalidTeam);
    }

    #[test]
    fn test_join_event_already_joined() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "player1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        let result = join_event(&mut state, 21, "player1", TEAM_COLOUR_RED, 0.0, 0.0);
        assert_eq!(result, JoinResult::AlreadyJoined);
    }

    #[test]
    fn test_join_event_team_full() {
        let mut state = SoccerState::default();
        for i in 0..11 {
            let name = format!("blue_{}", i);
            let result = join_event(&mut state, 21, &name, TEAM_COLOUR_BLUE, 0.0, 0.0);
            assert!(matches!(result, JoinResult::Ok { .. }));
        }
        // 12th blue player should be rejected.
        let result = join_event(&mut state, 21, "blue_11", TEAM_COLOUR_BLUE, 0.0, 0.0);
        assert_eq!(result, JoinResult::TeamFull);

        // Red team can still join.
        let result = join_event(&mut state, 21, "red_0", TEAM_COLOUR_RED, 0.0, 0.0);
        assert!(matches!(result, JoinResult::Ok { .. }));
    }

    #[test]
    fn test_join_event_room_full() {
        let mut state = SoccerState::default();
        for i in 0..11 {
            join_event(
                &mut state,
                21,
                &format!("blue_{}", i),
                TEAM_COLOUR_BLUE,
                0.0,
                0.0,
            );
        }
        for i in 0..11 {
            join_event(
                &mut state,
                21,
                &format!("red_{}", i),
                TEAM_COLOUR_RED,
                0.0,
                0.0,
            );
        }
        // 23rd player should fail.
        let result = join_event(&mut state, 21, "extra", TEAM_COLOUR_BLUE, 0.0, 0.0);
        // Should fail as either TeamFull or RoomFull
        assert!(result == JoinResult::TeamFull || result == JoinResult::RoomFull);
    }

    #[test]
    fn test_start_match_both_teams() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);

        let started = start_match(&mut state, 21, "blue1");
        assert!(started);

        let room = state.get_room(21).unwrap();
        assert!(room.is_active());
        assert_eq!(room.match_time, 600);
    }

    #[test]
    fn test_start_match_one_team_only() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);

        // Only blue team has players — should not start.
        let started = start_match(&mut state, 21, "blue1");
        assert!(!started);
    }

    #[test]
    fn test_start_match_not_registered() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);

        // Non-registered player cannot start.
        let started = start_match(&mut state, 21, "stranger");
        assert!(!started);
    }

    #[test]
    fn test_start_match_already_active() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);
        start_match(&mut state, 21, "blue1");

        // Second start should fail.
        let started = start_match(&mut state, 21, "red1");
        assert!(!started);
    }

    #[test]
    fn test_timer_tick_match_start() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);
        start_match(&mut state, 21, "blue1");

        let room = state.get_room_mut(21).unwrap();
        let result = timer_tick(room);
        assert!(matches!(result, TickResult::MatchStart { time: 600 }));
        assert_eq!(room.match_time, 599);
    }

    #[test]
    fn test_timer_tick_ball_check() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);
        start_match(&mut state, 21, "blue1");

        let room = state.get_room_mut(21).unwrap();
        // First tick (time=600 -> MatchStart).
        timer_tick(room);
        // Second tick (time=599 -> BallCheck).
        let result = timer_tick(room);
        assert_eq!(result, TickResult::BallCheck);
        assert_eq!(room.match_time, 598);
    }

    #[test]
    fn test_timer_tick_match_end() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);
        start_match(&mut state, 21, "blue1");

        let room = state.get_room_mut(21).unwrap();
        // Fast-forward to time=1.
        room.match_time = 1;
        room.blue_goals = 3;
        room.red_goals = 1;
        let result = timer_tick(room);
        assert_eq!(
            result,
            TickResult::MatchEnd {
                blue_goals: 3,
                red_goals: 1,
            }
        );
        assert!(!room.is_active());
        assert!(room.is_cooldown());
        assert_eq!(room.cooldown_ticks, COOLDOWN_TICKS);
    }

    #[test]
    fn test_timer_tick_cooldown_and_done() {
        let mut room = SoccerRoom {
            timer_flag: true,
            cooldown_ticks: 2,
            ..Default::default()
        };

        let result = timer_tick(&mut room);
        assert!(matches!(result, TickResult::Cooldown { remaining: 2 }));
        assert_eq!(room.cooldown_ticks, 1);

        let result = timer_tick(&mut room);
        assert_eq!(result, TickResult::CooldownDone);
        assert_eq!(room.cooldown_ticks, 0);
    }

    #[test]
    fn test_timer_tick_idle() {
        let mut room = SoccerRoom::default();
        let result = timer_tick(&mut room);
        assert_eq!(result, TickResult::Idle);
    }

    #[test]
    fn test_record_goal_blue_scores() {
        let mut room = SoccerRoom::default();
        // Ball in red goal → blue scores.
        let (blue, red) = record_goal(&mut room, TEAM_COLOUR_RED);
        assert_eq!(blue, 1);
        assert_eq!(red, 0);
    }

    #[test]
    fn test_record_goal_red_scores() {
        let mut room = SoccerRoom::default();
        // Ball in blue goal → red scores.
        let (blue, red) = record_goal(&mut room, TEAM_COLOUR_BLUE);
        assert_eq!(blue, 0);
        assert_eq!(red, 1);
    }

    #[test]
    fn test_determine_winner() {
        assert_eq!(determine_winner(3, 1), TEAM_COLOUR_BLUE);
        assert_eq!(determine_winner(1, 3), TEAM_COLOUR_RED);
        assert_eq!(determine_winner(2, 2), TEAM_COLOUR_NONE);
        assert_eq!(determine_winner(0, 0), TEAM_COLOUR_NONE);
    }

    #[test]
    fn test_end_teleport_position() {
        assert_eq!(end_teleport_position(TEAM_COLOUR_BLUE), (639.0, 194.0));
        assert_eq!(end_teleport_position(TEAM_COLOUR_RED), (703.0, 127.0));
        assert_eq!(end_teleport_position(TEAM_COLOUR_NONE), (672.0, 160.0));
    }

    #[test]
    fn test_remove_user() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);

        let room = state.get_room_mut(21).unwrap();
        assert_eq!(room.blue_count, 1);
        assert_eq!(room.total_user_count(), 2);

        remove_user(room, "blue1");
        assert_eq!(room.blue_count, 0);
        assert_eq!(room.red_count, 1);
        assert_eq!(room.total_user_count(), 1);

        // Removing non-existent user does nothing.
        remove_user(room, "blue1");
        assert_eq!(room.total_user_count(), 1);
    }

    #[test]
    fn test_is_soccer_user() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);

        assert!(is_soccer_user(&state, 21, "blue1"));
        assert!(!is_soccer_user(&state, 21, "nobody"));
        assert!(!is_soccer_user(&state, 22, "blue1")); // different zone
    }

    #[test]
    fn test_is_player_in_soccer() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);

        // On the field.
        assert!(is_player_in_soccer(&state, 21, "blue1", 672.0, 160.0));
        // Off the field.
        assert!(!is_player_in_soccer(&state, 21, "blue1", 600.0, 160.0));
        // Not registered.
        assert!(!is_player_in_soccer(&state, 21, "nobody", 672.0, 160.0));
        // Not moradon.
        assert!(!is_player_in_soccer(&state, 1, "blue1", 672.0, 160.0));
    }

    #[test]
    fn test_room_clean() {
        let mut state = SoccerState::default();
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);
        start_match(&mut state, 21, "blue1");

        let room = state.get_room_mut(21).unwrap();
        room.blue_goals = 5;
        room.red_goals = 3;
        room.clean();

        assert!(!room.is_active());
        assert!(!room.is_cooldown());
        assert_eq!(room.blue_count, 0);
        assert_eq!(room.red_count, 0);
        assert_eq!(room.blue_goals, 0);
        assert_eq!(room.red_goals, 0);
        assert_eq!(room.total_user_count(), 0);
        assert_eq!(room.match_time, 0);
    }

    #[test]
    fn test_full_match_lifecycle() {
        let mut state = SoccerState::default();

        // Players join.
        join_event(&mut state, 21, "blue1", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "red1", TEAM_COLOUR_RED, 0.0, 0.0);

        // Start match.
        assert!(start_match(&mut state, 21, "blue1"));

        // First tick (MatchStart).
        let room = state.get_room_mut(21).unwrap();
        assert!(matches!(timer_tick(room), TickResult::MatchStart { .. }));

        // Simulate a few ticks.
        for _ in 0..5 {
            assert_eq!(timer_tick(room), TickResult::BallCheck);
        }

        // Score some goals.
        record_goal(room, TEAM_COLOUR_RED); // Blue scores
        record_goal(room, TEAM_COLOUR_BLUE); // Red scores
        record_goal(room, TEAM_COLOUR_RED); // Blue scores again
        assert_eq!(room.blue_goals, 2);
        assert_eq!(room.red_goals, 1);

        // Fast-forward to end.
        room.match_time = 1;
        let result = timer_tick(room);
        assert_eq!(
            result,
            TickResult::MatchEnd {
                blue_goals: 2,
                red_goals: 1,
            }
        );

        // Determine winner.
        assert_eq!(determine_winner(2, 1), TEAM_COLOUR_BLUE);

        // Cooldown.
        for i in (2..=COOLDOWN_TICKS).rev() {
            let result = timer_tick(room);
            assert!(matches!(result, TickResult::Cooldown { remaining } if remaining == i));
        }
        assert_eq!(timer_tick(room), TickResult::CooldownDone);

        // Clean up.
        room.clean();
        assert_eq!(timer_tick(room), TickResult::Idle);
    }

    #[test]
    fn test_build_timer_packet() {
        let pkt = build_timer_packet(600);
        assert_eq!(pkt.opcode, Opcode::WizMining as u8);
        assert_eq!(pkt.data[0], SOCCER_EVENT_SUB);
        assert_eq!(pkt.data[1], 0x02);
        let timer = u16::from_le_bytes([pkt.data[2], pkt.data[3]]);
        assert_eq!(timer, 600);
    }

    #[test]
    fn test_build_goal_packet() {
        let pkt = build_goal_packet(100, TEAM_COLOUR_RED, 2, 1);
        assert_eq!(pkt.opcode, Opcode::WizMining as u8);
        assert_eq!(pkt.data[0], SOCCER_EVENT_SUB);
        assert_eq!(pkt.data[1], 0x01);
        // socket_id as u32 LE
        let sid = u32::from_le_bytes([pkt.data[2], pkt.data[3], pkt.data[4], pkt.data[5]]);
        assert_eq!(sid, 100);
        assert_eq!(pkt.data[6], TEAM_COLOUR_RED); // goal_zone
        assert_eq!(pkt.data[7], TEAM_COLOUR_RED); // goal_zone (written twice)
        assert_eq!(pkt.data[8], 2); // blue_goals
        assert_eq!(pkt.data[9], 1); // red_goals
    }

    #[test]
    fn test_build_end_packet() {
        let pkt = build_end_packet(TEAM_COLOUR_BLUE, 3, 1);
        assert_eq!(pkt.opcode, Opcode::WizMining as u8);
        assert_eq!(pkt.data[0], SOCCER_EVENT_SUB);
        assert_eq!(pkt.data[1], 0x04);
        assert_eq!(pkt.data[2], TEAM_COLOUR_BLUE);
        assert_eq!(pkt.data[3], 3); // blue_goals
        assert_eq!(pkt.data[4], 1); // red_goals
    }

    #[test]
    fn test_build_kick_response_success() {
        let (pkt, broadcast) = build_kick_response(42, false);
        assert!(broadcast);
        assert_eq!(pkt.opcode, Opcode::WizMining as u8);
        assert_eq!(pkt.data[0], SOCCER_EVENT_SUB);
        let result = u16::from_le_bytes([pkt.data[1], pkt.data[2]]);
        assert_eq!(result, 1); // success
        let sid = u32::from_le_bytes([pkt.data[3], pkt.data[4], pkt.data[5], pkt.data[6]]);
        assert_eq!(sid, 42);
    }

    #[test]
    fn test_build_kick_response_already_mining() {
        let (pkt, broadcast) = build_kick_response(42, true);
        assert!(!broadcast);
        assert_eq!(pkt.data[0], SOCCER_EVENT_SUB);
        let result = u16::from_le_bytes([pkt.data[1], pkt.data[2]]);
        assert_eq!(result, 2); // already mining
    }

    #[test]
    fn test_multiple_zones_independent() {
        let mut state = SoccerState::default();

        // Join zone 21.
        join_event(&mut state, 21, "player_a", TEAM_COLOUR_BLUE, 0.0, 0.0);
        join_event(&mut state, 21, "player_b", TEAM_COLOUR_RED, 0.0, 0.0);

        // Join zone 22.
        join_event(&mut state, 22, "player_c", TEAM_COLOUR_RED, 0.0, 0.0);

        let room21 = state.get_room(21).unwrap();
        assert_eq!(room21.total_user_count(), 2);

        let room22 = state.get_room(22).unwrap();
        assert_eq!(room22.total_user_count(), 1);

        // Users in zone 21 are not in zone 22.
        assert!(is_soccer_user(&state, 21, "player_a"));
        assert!(!is_soccer_user(&state, 22, "player_a"));
    }

    #[test]
    fn test_geometry_red_goal_boundary() {
        // Exactly on boundary (exclusive, should NOT be in goal zone).
        assert_ne!(check_ball_position(21, 661.0, 115.0), TEAM_COLOUR_RED);
        assert_ne!(check_ball_position(21, 681.0, 115.0), TEAM_COLOUR_RED);
        assert_ne!(check_ball_position(21, 670.0, 108.0), TEAM_COLOUR_RED);
        assert_ne!(check_ball_position(21, 670.0, 120.0), TEAM_COLOUR_RED);

        // Just inside.
        assert_eq!(check_ball_position(21, 661.5, 110.0), TEAM_COLOUR_RED);
        assert_eq!(check_ball_position(21, 680.5, 119.5), TEAM_COLOUR_RED);
    }

    #[test]
    fn test_geometry_blue_goal_boundary() {
        // Exactly on boundary (exclusive).
        assert_ne!(check_ball_position(21, 661.0, 203.0), TEAM_COLOUR_BLUE);
        assert_ne!(check_ball_position(21, 681.0, 203.0), TEAM_COLOUR_BLUE);
        assert_ne!(check_ball_position(21, 670.0, 199.0), TEAM_COLOUR_BLUE);
        assert_ne!(check_ball_position(21, 670.0, 208.0), TEAM_COLOUR_BLUE);

        // Just inside.
        assert_eq!(check_ball_position(21, 661.5, 200.0), TEAM_COLOUR_BLUE);
        assert_eq!(check_ball_position(21, 680.5, 207.5), TEAM_COLOUR_BLUE);
    }
}
