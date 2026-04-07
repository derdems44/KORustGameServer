//! Monster Stone instance system.
//!
//! C++ Reference: `MonsterStoneSystem.cpp` (562 lines)
//!
//! A player activates a Monster Stone item to enter a private instance (zones 81-83)
//! with waves of monsters. Killing the boss ends the instance after a 20-second grace.
//! If the timer expires (30 minutes), the player is kicked to Moradon.
//!
//! ## Flow
//!
//! 1. Player uses a Monster Stone item (WIZ_EVENT, sub-opcode 6)
//! 2. Server allocates a room from the pool (max 750)
//! 3. Player (+ party in new mode) teleported to zone 81/82/83
//! 4. Monsters spawned with the room's event_room ID
//! 5. Boss kill → 20-second grace → auto-reset
//! 6. Timer expiry (30 min) → auto-reset
//! 7. Any user logout/zone-exit → entire room destroyed

use ko_protocol::{Opcode, Packet};

// ── Constants ────────────────────────────────────────────────────────────────

/// Maximum number of monster stone rooms.
///
/// C++ Reference: `GameDefine.h:5` — `#define MAX_MONSTER_STONE_ROOM 750`
pub const MAX_ROOMS: usize = 750;

/// Room duration in seconds (30 minutes).
///
/// C++ Reference: `Define.h:93` — `#define MONSTER_STONE_TIME 1800`
pub const ROOM_DURATION_SECS: u64 = 1800;

/// NPC despawn duration in seconds.
///
/// C++ Reference: `Define.h:94` — `#define MONSTER_STONE_DEAD_TIME 1800`
pub const NPC_DESPAWN_SECS: u16 = 1800;

/// Grace period after boss kill (seconds).
///
/// C++ Reference: `Define.h:95` — `#define MONSTER_STONE_FINISH_TIME 20`
pub const FINISH_GRACE_SECS: u64 = 20;

pub use crate::world::types::{ZONE_MORADON, ZONE_STONE1, ZONE_STONE2, ZONE_STONE3};

/// WIZ_EVENT sub-opcode for Monster Stone activation.
///
/// C++ Reference: `packets.h` — `MONSTER_STONE = 6`
pub const SUB_OPCODE_MONSTER_STONE: u8 = 6;

/// WIZ_BIFROST sub-opcode for Monster Stone timer display.
///
/// C++ Reference: `packets.h` — `MONSTER_SQUARD = 5`
pub const SUB_OPCODE_MONSTER_SQUARD: u8 = 5;

/// WIZ_EVENT sub-opcode for temple event finish.
pub const SUB_OPCODE_TEMPLE_EVENT_FINISH: u8 = 10;

/// Valid Monster Stone item IDs.
///
/// C++ Reference: `MonsterStoneSystem.cpp:33-34`
pub const ITEM_STONE1: u32 = 300_144_036;
pub const ITEM_STONE2: u32 = 300_145_037;
pub const ITEM_STONE3: u32 = 300_146_038;
pub const ITEM_UNIVERSAL: u32 = 900_144_023;

/// SpawnEventType for Monster Stone NPCs.
///
/// C++ Reference: `GameDefine.h` — `SpawnEventType::MonsterStone = 8`
pub const SPAWN_EVENT_TYPE_MONSTER_STONE: u16 = 8;

// ── Zone Helper ──────────────────────────────────────────────────────────────

/// Check if a zone ID is a Monster Stone zone (81, 82, or 83).
///
/// C++ Reference: `Unit.h` — `isInMonsterStoneZone()`
pub fn is_monster_stone_zone(zone_id: u16) -> bool {
    zone_id == ZONE_STONE1 || zone_id == ZONE_STONE2 || zone_id == ZONE_STONE3
}

// ── Room State ───────────────────────────────────────────────────────────────

/// State of a single Monster Stone room.
///
/// C++ Reference: `_MONSTER_STONE_INFO` in `GameDefine.h:4909-4946`
#[derive(Debug, Clone)]
pub struct MonsterStoneRoom {
    /// Room index (0-based). Set once at initialization.
    pub room_id: u16,
    /// Whether this room slot is available.
    pub usable: bool,
    /// Whether the room is currently active (has players).
    pub active: bool,
    /// Zone ID (81/82/83) this room instance uses.
    pub zone_id: u8,
    /// Monster family index (1-13).
    pub monster_family: u16,
    /// Zone the player was in before entering (for reference).
    pub start_zone_id: u16,
    /// Unix timestamp when the room was activated.
    pub start_time: u64,
    /// Unix timestamp when the room expires (start + 1800).
    pub finish_time: u64,
    /// Unix timestamp for boss-kill grace period expiry (0 = not set).
    pub waiting_time: u64,
    /// Whether the boss has been killed.
    pub boss_killed: bool,
    /// Session IDs of players in this room.
    pub users: Vec<u16>,
}

impl MonsterStoneRoom {
    /// Create a new room with the given slot index.
    pub fn new(room_id: u16) -> Self {
        Self {
            room_id,
            usable: true,
            active: false,
            zone_id: 0,
            monster_family: 0,
            start_zone_id: 0,
            start_time: 0,
            finish_time: 0,
            waiting_time: 0,
            boss_killed: false,
            users: Vec::new(),
        }
    }

    /// Reset the room state, preserving room_id.
    ///
    /// C++ Reference: `_MONSTER_STONE_INFO::reset()` — preserves `roomid`.
    pub fn reset(&mut self) {
        self.active = false;
        self.zone_id = 0;
        self.monster_family = 0;
        self.start_zone_id = 0;
        self.start_time = 0;
        self.finish_time = 0;
        self.waiting_time = 0;
        self.boss_killed = false;
        self.users.clear();
    }

    /// Check if this room can be allocated (usable, not active, no users).
    ///
    /// C++ Reference: `MonsterStoneSystem.cpp:50-54`
    pub fn is_available(&self) -> bool {
        self.usable && !self.active && self.users.is_empty()
    }
}

// ── Room Pool Manager ────────────────────────────────────────────────────────

/// Manages the pool of Monster Stone rooms.
///
/// C++ Reference: `m_TempleEventMonsterStoneRoomList[MAX_MONSTER_STONE_ROOM]`
#[derive(Debug)]
pub struct MonsterStoneManager {
    rooms: Vec<MonsterStoneRoom>,
}

impl Default for MonsterStoneManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MonsterStoneManager {
    /// Create a new manager with `MAX_ROOMS` room slots.
    pub fn new() -> Self {
        let rooms = (0..MAX_ROOMS as u16).map(MonsterStoneRoom::new).collect();
        Self { rooms }
    }

    /// Allocate the first available room. Returns the room index (0-based).
    ///
    /// C++ Reference: `MonsterStoneSystem.cpp:49-59` — first-fit linear scan.
    pub fn allocate_room(&mut self) -> Option<u16> {
        for room in &self.rooms {
            if room.is_available() {
                return Some(room.room_id);
            }
        }
        None
    }

    /// Get a reference to a room by index.
    pub fn get_room(&self, room_id: u16) -> Option<&MonsterStoneRoom> {
        self.rooms.get(room_id as usize)
    }

    /// Get a mutable reference to a room by index.
    pub fn get_room_mut(&mut self, room_id: u16) -> Option<&mut MonsterStoneRoom> {
        self.rooms.get_mut(room_id as usize)
    }

    /// Activate a room with the given parameters.
    ///
    /// C++ Reference: `MonsterStoneSystem.cpp:149-156`
    pub fn activate_room(
        &mut self,
        room_id: u16,
        zone_id: u8,
        monster_family: u16,
        start_zone_id: u16,
        now: u64,
    ) -> bool {
        let Some(room) = self.rooms.get_mut(room_id as usize) else {
            return false;
        };
        if !room.is_available() {
            return false;
        }
        room.active = true;
        room.zone_id = zone_id;
        room.monster_family = monster_family;
        room.start_zone_id = start_zone_id;
        room.start_time = now;
        room.finish_time = now + ROOM_DURATION_SECS;
        room.boss_killed = false;
        room.waiting_time = 0;
        true
    }

    /// Process timer tick for all active rooms. Returns indices of rooms that expired.
    ///
    /// C++ Reference: `TempleMonsterStoneTimer()` in `MonsterStoneSystem.cpp:399-418`
    pub fn timer_tick(&mut self, now: u64) -> Vec<u16> {
        let mut expired = Vec::new();
        for room in &mut self.rooms {
            if !room.usable || !room.active {
                continue;
            }

            let time_expired = now >= room.finish_time;
            let wait_expired =
                room.boss_killed && room.waiting_time > 0 && now >= room.waiting_time;

            if (!room.boss_killed && time_expired) || wait_expired {
                room.active = false;
                expired.push(room.room_id);
            }
        }
        expired
    }

    /// Mark boss killed in a room and set the grace period.
    ///
    /// C++ Reference: `MonsterStoneKillProcess()` in `MonsterStoneSystem.cpp:508-540`
    pub fn boss_killed(&mut self, room_id: u16, now: u64) -> bool {
        let Some(room) = self.rooms.get_mut(room_id as usize) else {
            return false;
        };
        if !room.active || room.boss_killed {
            return false;
        }
        room.boss_killed = true;
        room.waiting_time = now + FINISH_GRACE_SECS;
        true
    }

    /// Reset a room and return the list of user session IDs that were in it.
    ///
    /// C++ Reference: `TempleMonsterStoneAutoResetRoom()` in `MonsterStoneSystem.cpp:422-438`
    pub fn reset_room(&mut self, room_id: u16) -> Vec<u16> {
        let Some(room) = self.rooms.get_mut(room_id as usize) else {
            return Vec::new();
        };
        let users = room.users.clone();
        room.reset();
        users
    }

    /// Add a user to a room.
    pub fn add_user(&mut self, room_id: u16, session_id: u16) {
        if let Some(room) = self.rooms.get_mut(room_id as usize) {
            if !room.users.contains(&session_id) {
                room.users.push(session_id);
            }
        }
    }

    /// Remove a user from a room. Returns true if the room is now empty.
    pub fn remove_user(&mut self, room_id: u16, session_id: u16) -> bool {
        if let Some(room) = self.rooms.get_mut(room_id as usize) {
            room.users.retain(|&id| id != session_id);
            return room.users.is_empty();
        }
        false
    }

    /// Find which room a user session is in. Returns room index or None.
    pub fn find_user_room(&self, session_id: u16) -> Option<u16> {
        for room in &self.rooms {
            if room.active && room.users.contains(&session_id) {
                return Some(room.room_id);
            }
        }
        None
    }
}

// ── Packet Builders ──────────────────────────────────────────────────────────

/// Build the WIZ_BIFROST Monster Stone timer packet.
///
/// C++ Reference: `MonsterStoneTimerScreen()` line 545-548
///
/// `WIZ_BIFROST | uint8(MONSTER_SQUARD=5) | uint16(time_secs)`
pub fn build_timer_packet(time_secs: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizBifrost as u8);
    pkt.write_u8(SUB_OPCODE_MONSTER_SQUARD);
    pkt.write_u16(time_secs);
    pkt
}

/// Build the WIZ_SELECT_MSG timer display packet.
///
/// C++ Reference: `MonsterStoneTimerScreen()` line 550-560
pub fn build_select_msg_timer(time_secs: u16) -> Packet {
    let mut pkt = Packet::new(Opcode::WizSelectMsg as u8);
    pkt.write_u32(0);
    pkt.write_u8(7);
    pkt.write_u64(0);
    pkt.write_u8(9);
    pkt.write_u16(0);
    pkt.write_u8(0);
    pkt.write_u8(11);
    pkt.write_u16(time_secs);
    pkt.write_u16(0);
    pkt
}

/// Build the WIZ_EVENT Monster Stone fail response.
///
/// C++ Reference: `SendMonsterStoneFail()` lines 3-7
///
/// `WIZ_EVENT | uint8(MONSTER_STONE=6) | uint8(error_id)`
pub fn build_fail_packet(error_id: u8) -> Packet {
    let mut pkt = Packet::new(Opcode::WizEvent as u8);
    pkt.write_u8(SUB_OPCODE_MONSTER_STONE);
    pkt.write_u8(error_id);
    pkt
}

/// Build the boss kill finish notification packets.
///
/// C++ Reference: `MonsterStoneKillProcess()` lines 524-537
///
/// Returns (event_finish_pkt, quest_pkt).
pub fn build_boss_kill_packets() -> (Packet, Packet) {
    // Packet 1: WIZ_EVENT TEMPLE_EVENT_FINISH
    let mut finish_pkt = Packet::new(Opcode::WizEvent as u8);
    finish_pkt.write_u8(SUB_OPCODE_TEMPLE_EVENT_FINISH);
    finish_pkt.write_u8(0x11);
    finish_pkt.write_u8(0x00);
    finish_pkt.write_u8(0x65);
    finish_pkt.write_u8(0x14);
    finish_pkt.write_u32(0x00);

    // Packet 2: WIZ_QUEST completion
    let mut quest_pkt = Packet::new(Opcode::WizQuest as u8);
    quest_pkt.write_u8(2);
    quest_pkt.write_u16(209);
    quest_pkt.write_u8(0);

    (finish_pkt, quest_pkt)
}

/// Determine zone and family for the universal stone (item 900144023) based on level.
///
/// C++ Reference: `MonsterStoneSystem.cpp:251-308`
pub fn universal_stone_zone_family(level: u8) -> Option<(u8, u16)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    match level {
        20..=29 => Some((ZONE_STONE1 as u8, 1)),
        30..=35 => Some((ZONE_STONE1 as u8, 2)),
        36..=40 => Some((ZONE_STONE1 as u8, 3)),
        41..=46 => Some((ZONE_STONE1 as u8, 4)),
        47..=55 => {
            let family = if rng.gen_bool(0.5) { 4 } else { 5 };
            let zone = if family <= 4 {
                ZONE_STONE1 as u8
            } else {
                ZONE_STONE2 as u8
            };
            Some((zone, family))
        }
        56..=60 => Some((ZONE_STONE2 as u8, rng.gen_range(6..=8))),
        61..=66 => Some((ZONE_STONE2 as u8, rng.gen_range(8..=9))),
        67..=70 => {
            let family: u16 = rng.gen_range(9..=10);
            let zone = if family == 9 {
                ZONE_STONE2 as u8
            } else {
                ZONE_STONE3 as u8
            };
            Some((zone, family))
        }
        71..=74 => Some((ZONE_STONE3 as u8, rng.gen_range(10..=12))),
        75..=u8::MAX => Some((ZONE_STONE3 as u8, 13)),
        _ => None, // Below level 20
    }
}

/// Determine zone for a specific stone item in new_monsterstone mode.
///
/// C++ Reference: `MonsterStoneSystem.cpp:82-88`
pub fn item_to_zone(item_id: u32) -> Option<u8> {
    match item_id {
        ITEM_STONE1 => Some(ZONE_STONE1 as u8),
        ITEM_STONE2 => Some(ZONE_STONE2 as u8),
        ITEM_STONE3 => Some(ZONE_STONE3 as u8),
        _ => None,
    }
}

/// Random family for a zone in new_monsterstone mode.
///
/// C++ Reference: `MonsterStoneSystem.cpp:89-94`
pub fn random_family_for_zone(zone_id: u8) -> u16 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    match zone_id {
        81 => rng.gen_range(1..=4),
        82 => rng.gen_range(5..=9),
        83 => rng.gen_range(10..=13),
        _ => 1,
    }
}

/// Check if an item ID is a valid Monster Stone item.
pub fn is_monster_stone_item(item_id: u32) -> bool {
    matches!(
        item_id,
        ITEM_STONE1 | ITEM_STONE2 | ITEM_STONE3 | ITEM_UNIVERSAL
    )
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ZONE_STONE1, 81);
        assert_eq!(ZONE_STONE2, 82);
        assert_eq!(ZONE_STONE3, 83);
        assert_eq!(MAX_ROOMS, 750);
        assert_eq!(ROOM_DURATION_SECS, 1800);
        assert_eq!(FINISH_GRACE_SECS, 20);
    }

    #[test]
    fn test_is_monster_stone_zone() {
        assert!(is_monster_stone_zone(81));
        assert!(is_monster_stone_zone(82));
        assert!(is_monster_stone_zone(83));
        assert!(!is_monster_stone_zone(84)); // BDW
        assert!(!is_monster_stone_zone(21)); // Moradon
        assert!(!is_monster_stone_zone(0));
    }

    #[test]
    fn test_room_creation_and_availability() {
        let room = MonsterStoneRoom::new(42);
        assert_eq!(room.room_id, 42);
        assert!(room.usable);
        assert!(!room.active);
        assert!(room.is_available());
    }

    #[test]
    fn test_room_reset_preserves_id() {
        let mut room = MonsterStoneRoom::new(7);
        room.active = true;
        room.zone_id = 81;
        room.users.push(100);

        room.reset();
        assert_eq!(room.room_id, 7); // preserved
        assert!(!room.active);
        assert_eq!(room.zone_id, 0);
        assert!(room.users.is_empty());
    }

    #[test]
    fn test_manager_allocate_room() {
        let mut mgr = MonsterStoneManager::new();
        assert_eq!(mgr.rooms.len(), MAX_ROOMS);

        let room_id = mgr.allocate_room().unwrap();
        assert_eq!(room_id, 0); // first available

        // Activate it
        assert!(mgr.activate_room(0, 81, 3, 21, 1000));

        // Next allocation should be room 1
        let next = mgr.allocate_room().unwrap();
        assert_eq!(next, 1);
    }

    #[test]
    fn test_manager_activate_room() {
        let mut mgr = MonsterStoneManager::new();
        assert!(mgr.activate_room(0, 82, 5, 21, 1000));

        let room = mgr.get_room(0).unwrap();
        assert!(room.active);
        assert_eq!(room.zone_id, 82);
        assert_eq!(room.monster_family, 5);
        assert_eq!(room.start_zone_id, 21);
        assert_eq!(room.start_time, 1000);
        assert_eq!(room.finish_time, 1000 + ROOM_DURATION_SECS);
        assert!(!room.boss_killed);

        // Cannot activate again
        assert!(!mgr.activate_room(0, 83, 10, 21, 2000));
    }

    #[test]
    fn test_timer_tick_time_expired() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 81, 1, 21, 1000);
        mgr.add_user(0, 100);

        // Before finish time — no expiry
        let expired = mgr.timer_tick(1000 + ROOM_DURATION_SECS - 1);
        assert!(expired.is_empty());

        // At finish time — expires
        let expired = mgr.timer_tick(1000 + ROOM_DURATION_SECS);
        assert_eq!(expired, vec![0]);
        assert!(!mgr.get_room(0).unwrap().active);
    }

    #[test]
    fn test_timer_tick_boss_killed_grace() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 83, 13, 21, 1000);

        // Boss killed at t=1500
        assert!(mgr.boss_killed(0, 1500));
        let room = mgr.get_room(0).unwrap();
        assert!(room.boss_killed);
        assert_eq!(room.waiting_time, 1500 + FINISH_GRACE_SECS);

        // Before grace expires — no expiry
        let expired = mgr.timer_tick(1500 + FINISH_GRACE_SECS - 1);
        assert!(expired.is_empty());

        // Grace expired
        let expired = mgr.timer_tick(1500 + FINISH_GRACE_SECS);
        assert_eq!(expired, vec![0]);
    }

    #[test]
    fn test_reset_room_returns_users() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 81, 1, 21, 1000);
        mgr.add_user(0, 100);
        mgr.add_user(0, 200);

        let users = mgr.reset_room(0);
        assert_eq!(users, vec![100, 200]);
        assert!(mgr.get_room(0).unwrap().users.is_empty());
        assert!(!mgr.get_room(0).unwrap().active);
    }

    #[test]
    fn test_find_user_room() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(5, 82, 7, 21, 1000);
        mgr.add_user(5, 42);

        assert_eq!(mgr.find_user_room(42), Some(5));
        assert_eq!(mgr.find_user_room(99), None);
    }

    #[test]
    fn test_build_timer_packet() {
        let pkt = build_timer_packet(1800);
        assert_eq!(pkt.data[0], SUB_OPCODE_MONSTER_SQUARD);
        // u16 LE: 1800 = 0x0708
        assert_eq!(pkt.data[1], 0x08);
        assert_eq!(pkt.data[2], 0x07);
    }

    #[test]
    fn test_build_fail_packet() {
        let pkt = build_fail_packet(9);
        assert_eq!(pkt.data[0], SUB_OPCODE_MONSTER_STONE);
        assert_eq!(pkt.data[1], 9);
    }

    #[test]
    fn test_build_boss_kill_packets() {
        let (finish, quest) = build_boss_kill_packets();
        assert_eq!(finish.data[0], SUB_OPCODE_TEMPLE_EVENT_FINISH);
        assert_eq!(finish.data[1], 0x11);
        assert_eq!(quest.data[0], 2);
    }

    #[test]
    fn test_is_monster_stone_item() {
        assert!(is_monster_stone_item(ITEM_STONE1));
        assert!(is_monster_stone_item(ITEM_STONE2));
        assert!(is_monster_stone_item(ITEM_STONE3));
        assert!(is_monster_stone_item(ITEM_UNIVERSAL));
        assert!(!is_monster_stone_item(12345));
    }

    #[test]
    fn test_item_to_zone() {
        assert_eq!(item_to_zone(ITEM_STONE1), Some(81));
        assert_eq!(item_to_zone(ITEM_STONE2), Some(82));
        assert_eq!(item_to_zone(ITEM_STONE3), Some(83));
        assert_eq!(item_to_zone(ITEM_UNIVERSAL), None);
    }

    #[test]
    fn test_universal_stone_zone_family_level_20() {
        let result = universal_stone_zone_family(20);
        assert!(result.is_some());
        let (zone, family) = result.unwrap();
        assert_eq!(zone, 81);
        assert_eq!(family, 1);
    }

    #[test]
    fn test_universal_stone_zone_family_level_75() {
        let (zone, family) = universal_stone_zone_family(75).unwrap();
        assert_eq!(zone, 83);
        assert_eq!(family, 13);
    }

    #[test]
    fn test_universal_stone_zone_family_too_low() {
        assert!(universal_stone_zone_family(19).is_none());
        assert!(universal_stone_zone_family(10).is_none());
    }

    #[test]
    fn test_random_family_for_zone() {
        for _ in 0..20 {
            let f = random_family_for_zone(81);
            assert!((1..=4).contains(&f));
        }
        for _ in 0..20 {
            let f = random_family_for_zone(82);
            assert!((5..=9).contains(&f));
        }
        for _ in 0..20 {
            let f = random_family_for_zone(83);
            assert!((10..=13).contains(&f));
        }
    }

    #[test]
    fn test_boss_killed_double_call() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 81, 1, 21, 1000);

        assert!(mgr.boss_killed(0, 1500));
        assert!(!mgr.boss_killed(0, 1600)); // already killed
    }

    #[test]
    fn test_remove_user() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 81, 1, 21, 1000);
        mgr.add_user(0, 100);
        mgr.add_user(0, 200);

        assert!(!mgr.remove_user(0, 100)); // room still has user 200
        assert!(mgr.remove_user(0, 200)); // room is now empty
    }

    #[test]
    fn test_add_user_no_duplicates() {
        let mut mgr = MonsterStoneManager::new();
        mgr.activate_room(0, 81, 1, 21, 1000);
        mgr.add_user(0, 100);
        mgr.add_user(0, 100); // duplicate

        assert_eq!(mgr.get_room(0).unwrap().users.len(), 1);
    }

    #[test]
    fn test_build_select_msg_timer() {
        let pkt = build_select_msg_timer(1800);
        assert_eq!(pkt.opcode, Opcode::WizSelectMsg as u8);
        assert!(!pkt.data.is_empty());
    }
}
