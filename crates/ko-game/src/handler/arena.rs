//! WIZ_PVP (0x88) handler — PvP Rival & Anger Gauge system.
//! ## Overview
//! The Rival system activates in Ardream / Ronark Land PvP zones:
//! 1. When a player is killed by an enemy, if the victim has no rival yet the
//!    killer becomes the victim's rival for 5 minutes (`RIVALRY_DURATION = 300s`).
//! 2. Each kill also increments the *anger gauge* (0-5).  The gauge is broadcast
//!    to the player's own client via `PVPUpdateHelmet` (5) so a helmet icon
//!    reflects the level.
//! 3. If the rival kills the same victim while the rivalry is still active, the
//!    kill counts as a *revenge kill*: the rival earns a 150 NP bonus
//!    (`RIVALRY_NP_BONUS`) and the rivalry is removed.
//! 4. On regene (and Draki Tower regene) the anger gauge resets to 0 via
//!    `PVPResetHelmet` (6).
//! ## WIZ_PVP Sub-opcodes
//! | Value | Name             | Direction      | Payload                                   |
//! |-------|------------------|----------------|-------------------------------------------|
//! | 1     | PVPAssignRival   | Server → Client| u32 rival_sid, u16, u16, u32 coins, u32 np, sbyte clan_name, sbyte name |
//! | 2     | PVPRemoveRival   | Server → Client| (no payload)                              |
//! | 5     | PVPUpdateHelmet  | Server → Client| u8 gauge, u8 is_full                      |
//! | 6     | PVPResetHelmet   | Server → Client| (no payload)                              |
//! Note: WIZ_PVP is entirely **server → client**. The client never sends this
//! opcode. The handler in mod.rs is a no-op pass-through for any stray packets.
//! ## Integration Points
//! * `attack.rs` — calls [`on_pvp_kill`] after a successful PvP kill in the
//!   rivalry zones.
//! * `regene.rs` — calls [`reset_anger_gauge`] on player regene.
//! * `world/session.rs` — `set_rival`, `remove_rival`, `update_anger_gauge`,
//!   `check_rivalry_expiry` world-state helpers.

use ko_protocol::{Opcode, Packet};
use tracing::debug;

use crate::world::{WorldState, ZONE_ARDREAM, ZONE_RONARK_LAND, ZONE_RONARK_LAND_BASE};
use crate::zone::SessionId;

// ── Sub-opcode constants (C++ packets.h:324-329) ────────────────────────────

/// PVPAssignRival — assigns killer as victim's rival.
pub const PVP_ASSIGN_RIVAL: u8 = 1;
/// PVPRemoveRival — clears rivalry (revenge kill or expiry).
pub const PVP_REMOVE_RIVAL: u8 = 2;
/// PVPUpdateHelmet — updates anger gauge display on helmet.
pub const PVP_UPDATE_HELMET: u8 = 5;
/// PVPResetHelmet — resets anger gauge to 0 (on regene).
pub const PVP_RESET_HELMET: u8 = 6;

// ── Constants (C++ GameDefine.h:1329-1333) ───────────────────────────────────

/// Rivalry duration in seconds (5 minutes).
pub const RIVALRY_DURATION: u64 = 300;

/// NP bonus for killing your rival (revenge kill).
pub const RIVALRY_NP_BONUS: u16 = 150;

/// Maximum anger gauge level.
pub const MAX_ANGER_GAUGE: u8 = 5;

// ── Zone helpers ─────────────────────────────────────────────────────────────

/// Returns `true` for zones where the rivalry (anger gauge / rival) system is
/// active.
/// `ZONE_ARDREAM`, `ZONE_RONARK_LAND`, `ZONE_RONARK_LAND_BASE` enable the rival.
pub fn is_rivalry_zone(zone_id: u16) -> bool {
    zone_id == ZONE_ARDREAM || zone_id == ZONE_RONARK_LAND || zone_id == ZONE_RONARK_LAND_BASE
}

// ── Core PvP kill integration ─────────────────────────────────────────────────

/// Called after every successful PvP kill in a rivalry-eligible zone.
/// Performs, in order:
/// 1. Increments the victim's anger gauge (capped at `MAX_ANGER_GAUGE`).
/// 2. Assigns the killer as the victim's rival if the victim has no active rival.
/// 3. Checks whether the victim was the killer's rival; if so, removes the
///    rivalry and returns `true` (caller should add `RIVALRY_NP_BONUS` NP).
/// # Returns
/// `true` when the kill was a *revenge kill* (killer had the victim as rival).
pub fn on_pvp_kill(
    world: &WorldState,
    killer_sid: SessionId,
    victim_sid: SessionId,
    zone_id: u16,
    now_secs: u64,
) -> bool {
    if !is_rivalry_zone(zone_id) {
        return false;
    }

    // Check if this is a revenge kill (killer had victim as rival)
    let is_revenge = world
        .with_session(killer_sid, |h| {
            h.character.as_ref().is_some_and(|ch| {
                ch.rival_id == victim_sid as i16 && now_secs < ch.rival_expiry_time
            })
        })
        .unwrap_or(false);

    // Increment victim's anger gauge
    let current_gauge = world
        .with_session(victim_sid, |h| {
            h.character.as_ref().map(|ch| ch.anger_gauge).unwrap_or(0)
        })
        .unwrap_or(0);

    if current_gauge < MAX_ANGER_GAUGE {
        world.update_anger_gauge(victim_sid, current_gauge + 1);
    }

    // Assign rival on victim (if no active rival)
    world.set_rival(victim_sid, killer_sid, now_secs);

    // If revenge kill: remove rivalry from killer
    if is_revenge {
        world.remove_rival(killer_sid);
        debug!(
            "[arena] Revenge kill: killer {} had victim {} as rival — rivalry removed",
            killer_sid, victim_sid
        );
    }

    is_revenge
}

/// Reset a player's anger gauge to 0 (called on regene).
pub fn reset_anger_gauge(world: &WorldState, sid: SessionId) {
    let current = world
        .with_session(sid, |h| {
            h.character.as_ref().map(|ch| ch.anger_gauge).unwrap_or(0)
        })
        .unwrap_or(0);

    if current > 0 {
        world.update_anger_gauge(sid, 0);
        debug!("[arena] Anger gauge reset for session {}", sid);
    }
}

// ── S2C Packet Builders ─────────────────────────────────────────────────────

/// Build a WIZ_PVP(PVPAssignRival=1) packet.
/// Format: [u8:1] [u32:rival_sid] [u16:1] [u16:1] [u32:coins] [u32:loyalty]
///         [sbyte:clan_name or u16:0] [sbyte:rival_name]
pub fn build_assign_rival_packet(
    rival_sid: u32,
    coins: u32,
    loyalty: u32,
    clan_name: Option<&str>,
    rival_name: &str,
) -> ko_protocol::Packet {
    let mut pkt = Packet::new(Opcode::WizPvp as u8);
    pkt.write_u8(PVP_ASSIGN_RIVAL);
    pkt.write_u32(rival_sid);
    pkt.write_u16(1);
    pkt.write_u16(1);
    pkt.write_u32(coins);
    pkt.write_u32(loyalty);
    match clan_name {
        Some(name) if !name.is_empty() => pkt.write_sbyte_string(name),
        _ => pkt.write_u16(0),
    }
    pkt.write_sbyte_string(rival_name);
    pkt
}

/// Build a WIZ_PVP(PVPRemoveRival=2) packet — no payload beyond sub-opcode.
pub fn build_remove_rival_packet() -> Packet {
    let mut pkt = Packet::new(Opcode::WizPvp as u8);
    pkt.write_u8(PVP_REMOVE_RIVAL);
    pkt
}

/// Build a WIZ_PVP(PVPUpdateHelmet=5) packet — anger gauge update.
/// Format: [u8:5] [u8:gauge] [u8:is_full]
pub fn build_update_helmet_packet(gauge: u8) -> Packet {
    let clamped = gauge.min(MAX_ANGER_GAUGE);
    let mut pkt = Packet::new(Opcode::WizPvp as u8);
    pkt.write_u8(PVP_UPDATE_HELMET);
    pkt.write_u8(clamped);
    pkt.write_u8(u8::from(clamped >= MAX_ANGER_GAUGE));
    pkt
}

/// Build a WIZ_PVP(PVPResetHelmet=6) packet — no payload beyond sub-opcode.
pub fn build_reset_helmet_packet() -> Packet {
    let mut pkt = Packet::new(Opcode::WizPvp as u8);
    pkt.write_u8(PVP_RESET_HELMET);
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;
    use ko_protocol::PacketReader;

    // ── Constant tests ────────────────────────────────────────────────────

    #[test]
    fn test_rivalry_constants() {
        // C++ GameDefine.h:1329-1333
        assert_eq!(RIVALRY_DURATION, 300);
        assert_eq!(RIVALRY_NP_BONUS, 150);
        assert_eq!(MAX_ANGER_GAUGE, 5);
    }

    // ── Zone classification tests ─────────────────────────────────────────

    #[test]
    fn test_rivalry_zone_ardream() {
        assert!(is_rivalry_zone(ZONE_ARDREAM));
    }

    #[test]
    fn test_rivalry_zone_ronark_land() {
        assert!(is_rivalry_zone(ZONE_RONARK_LAND));
    }

    #[test]
    fn test_rivalry_zone_ronark_land_base() {
        assert!(is_rivalry_zone(ZONE_RONARK_LAND_BASE));
    }

    #[test]
    fn test_non_rivalry_zones() {
        // Battle zones do not activate the rivalry system
        assert!(!is_rivalry_zone(21)); // Moradon
        assert!(!is_rivalry_zone(1)); // Elmorad
        assert!(!is_rivalry_zone(2)); // Karus
        assert!(!is_rivalry_zone(22)); // Battle
        assert!(!is_rivalry_zone(48)); // Arena (ZONE_ARENA)
    }

    // ── Packet format tests ───────────────────────────────────────────────

    /// PVPAssignRival packet: opcode 0x88, sub-op 1, u32 sid, u16, u16, u32 coins, u32 loyalty,
    /// then clan name (sbyte or u16:0), then rival name (sbyte).
    #[test]
    fn test_assign_rival_packet_no_clan() {
        let pkt = build_assign_rival_packet(42, 1000, 500, None, "Killer");

        assert_eq!(pkt.opcode, Opcode::WizPvp as u8);
        let mut r = PacketReader::new(&pkt.data);

        assert_eq!(r.read_u8().unwrap(), 1); // PVPAssignRival
        assert_eq!(r.read_u32().unwrap(), 42);
        assert_eq!(r.read_u16().unwrap(), 1);
        assert_eq!(r.read_u16().unwrap(), 1);
        assert_eq!(r.read_u32().unwrap(), 1000); // coins
        assert_eq!(r.read_u32().unwrap(), 500); // loyalty
        assert_eq!(r.read_u16().unwrap(), 0); // no clan (u16:0)
        assert_eq!(r.read_sbyte_string().unwrap(), "Killer");
    }

    /// PVPAssignRival packet with clan name uses sbyte string.
    #[test]
    fn test_assign_rival_packet_with_clan() {
        let pkt = build_assign_rival_packet(7, 2000, 300, Some("StormClan"), "Victor");

        assert_eq!(pkt.opcode, Opcode::WizPvp as u8);
        let mut r = PacketReader::new(&pkt.data);

        assert_eq!(r.read_u8().unwrap(), 1); // PVPAssignRival
        assert_eq!(r.read_u32().unwrap(), 7);
        r.read_u16().unwrap(); // skip 1
        r.read_u16().unwrap(); // skip 1
        r.read_u32().unwrap(); // skip coins
        r.read_u32().unwrap(); // skip loyalty
        assert_eq!(r.read_sbyte_string().unwrap(), "StormClan");
        assert_eq!(r.read_sbyte_string().unwrap(), "Victor");
    }

    /// PVPRemoveRival packet via builder: opcode 0x88, sub-op 2, no payload.
    #[test]
    fn test_remove_rival_packet_format() {
        let pkt = build_remove_rival_packet();

        assert_eq!(pkt.opcode, Opcode::WizPvp as u8);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_REMOVE_RIVAL);
        assert!(r.read_u8().is_none());
    }

    /// PVPUpdateHelmet via builder: opcode 0x88, sub-op 5, u8 gauge, u8 is_full.
    #[test]
    fn test_update_helmet_packet_gauge_3() {
        let pkt = build_update_helmet_packet(3);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_UPDATE_HELMET);
        assert_eq!(r.read_u8().unwrap(), 3);
        assert_eq!(r.read_u8().unwrap(), 0); // not full (3 < 5)
    }

    /// PVPUpdateHelmet at max gauge sets is_full = 1.
    #[test]
    fn test_update_helmet_packet_full_gauge() {
        let pkt = build_update_helmet_packet(MAX_ANGER_GAUGE);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_UPDATE_HELMET);
        assert_eq!(r.read_u8().unwrap(), MAX_ANGER_GAUGE);
        assert_eq!(r.read_u8().unwrap(), 1); // full
    }

    /// PVPResetHelmet via builder: opcode 0x88, sub-op 6, no payload.
    #[test]
    fn test_reset_helmet_packet_format() {
        let pkt = build_reset_helmet_packet();

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_RESET_HELMET);
        assert!(r.read_u8().is_none());
    }

    /// Verify sub-opcode constants match C++ packets.h:324-329.
    #[test]
    fn test_pvp_sub_opcode_values() {
        assert_eq!(PVP_ASSIGN_RIVAL, 1);
        assert_eq!(PVP_REMOVE_RIVAL, 2);
        assert_eq!(PVP_UPDATE_HELMET, 5);
        assert_eq!(PVP_RESET_HELMET, 6);
    }

    /// Verify opcode value matches C++ packets.h: `WIZ_PVP = 0x88`
    #[test]
    fn test_wiz_pvp_opcode_value() {
        assert_eq!(Opcode::WizPvp as u8, 0x88);
    }

    /// Anger gauge is capped at MAX_ANGER_GAUGE.
    #[test]
    fn test_max_anger_gauge_cap() {
        assert_eq!(MAX_ANGER_GAUGE, 5);
        let clamped = 6u8.min(MAX_ANGER_GAUGE);
        assert_eq!(clamped, 5);
    }

    /// Rivalry zone detection covers all three C++ zones.
    #[test]
    fn test_all_rivalry_zones_covered() {
        // ZONE_ARDREAM = 29, ZONE_RONARK_LAND = 31, ZONE_RONARK_LAND_BASE = 32
        // Verify all three match is_rivalry_zone
        let zones = [ZONE_ARDREAM, ZONE_RONARK_LAND, ZONE_RONARK_LAND_BASE];
        for z in zones {
            assert!(is_rivalry_zone(z), "Zone {} should be a rivalry zone", z);
        }
    }

    /// PVPAssignRival packet must start with sub-op 1 at offset 0.
    #[test]
    fn test_assign_rival_sub_opcode_position() {
        let pkt = build_assign_rival_packet(0, 0, 0, None, "X");
        // data[0] must be sub-opcode 1
        assert_eq!(pkt.data[0], 1);
    }

    /// Empty clan name is encoded as u16:0, not as an sbyte string.
    #[test]
    fn test_empty_clan_name_encoding() {
        let pkt = build_assign_rival_packet(1, 0, 0, Some(""), "Z");
        let mut r = PacketReader::new(&pkt.data);
        r.read_u8().unwrap(); // sub_op
        r.read_u32().unwrap(); // rival_sid
        r.read_u16().unwrap(); // 1
        r.read_u16().unwrap(); // 1
        r.read_u32().unwrap(); // coins
        r.read_u32().unwrap(); // loyalty
        assert_eq!(r.read_u16().unwrap(), 0); // Empty string → u16:0
    }

    /// Packet roundtrip: assign rival → read all fields back.
    #[test]
    fn test_assign_rival_full_roundtrip() {
        let sid: u32 = 999;
        let coins: u32 = 500_000;
        let loyalty: u32 = 12_345;
        let clan = "IronShield";
        let name = "Darkblade";

        let pkt = build_assign_rival_packet(sid, coins, loyalty, Some(clan), name);

        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_ASSIGN_RIVAL);
        assert_eq!(r.read_u32().unwrap(), sid);
        assert_eq!(r.read_u16().unwrap(), 1);
        assert_eq!(r.read_u16().unwrap(), 1);
        assert_eq!(r.read_u32().unwrap(), coins);
        assert_eq!(r.read_u32().unwrap(), loyalty);
        assert_eq!(r.read_sbyte_string().unwrap(), clan);
        assert_eq!(r.read_sbyte_string().unwrap(), name);
    }

    // ── New builder tests ────────────────────────────────────────────

    /// build_update_helmet_packet clamps gauge above MAX_ANGER_GAUGE.
    #[test]
    fn test_update_helmet_clamps_above_max() {
        let pkt = build_update_helmet_packet(10); // 10 > MAX(5) → clamped to 5
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_UPDATE_HELMET);
        assert_eq!(r.read_u8().unwrap(), MAX_ANGER_GAUGE); // clamped
        assert_eq!(r.read_u8().unwrap(), 1); // is_full
    }

    /// build_update_helmet_packet gauge=1 → is_full=0.
    #[test]
    fn test_update_helmet_gauge_1_not_full() {
        let pkt = build_update_helmet_packet(1);
        let mut r = PacketReader::new(&pkt.data);
        assert_eq!(r.read_u8().unwrap(), PVP_UPDATE_HELMET);
        assert_eq!(r.read_u8().unwrap(), 1);
        assert_eq!(r.read_u8().unwrap(), 0); // not full
        assert_eq!(r.remaining(), 0);
    }

    /// build_remove_rival_packet data length is exactly 1 byte (sub-opcode only).
    #[test]
    fn test_remove_rival_data_length() {
        let pkt = build_remove_rival_packet();
        assert_eq!(pkt.data.len(), 1);
    }

    /// build_reset_helmet_packet data length is exactly 1 byte (sub-opcode only).
    #[test]
    fn test_reset_helmet_data_length() {
        let pkt = build_reset_helmet_packet();
        assert_eq!(pkt.data.len(), 1);
    }

    /// build_update_helmet_packet data length is exactly 3 bytes.
    #[test]
    fn test_update_helmet_data_length() {
        let pkt = build_update_helmet_packet(3);
        assert_eq!(pkt.data.len(), 3); // sub_op + gauge + is_full
    }
}
